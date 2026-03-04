use serde_json::Value;

use crate::betty_blocks::open_id_connect::types::ApiError;
use crate::params::build_query_string;
use crate::wasi::http::outgoing_handler;
use crate::wasi::http::types::{
    Fields, Method, OutgoingBody, OutgoingRequest, RequestOptions, Scheme,
};

impl ApiError {
    fn new(error: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            error_description: Some(description.into()),
            error_uri: None,
        }
    }
}

struct ParsedUrl {
    scheme: Scheme,
    authority: String,
    path_and_query: String,
}

impl ParsedUrl {
    fn parse(url: &str) -> Result<Self, ApiError> {
        let (scheme, rest) = if let Some(r) = url.strip_prefix("https://") {
            (Scheme::Https, r)
        } else if let Some(r) = url.strip_prefix("http://") {
            (Scheme::Http, r)
        } else {
            return Err(ApiError::new(
                "invalid_url",
                format!("Unsupported scheme in URL: {url}"),
            ));
        };

        let (authority, path_and_query) = match rest.find('/') {
            Some(i) => (rest[..i].to_string(), rest[i..].to_string()),
            None => (rest.to_string(), "/".to_string()),
        };

        Ok(Self {
            scheme,
            authority,
            path_and_query,
        })
    }
}

fn read_body(body: crate::wasi::http::types::IncomingBody) -> Result<Vec<u8>, ApiError> {
    let stream = body
        .stream()
        .expect("body stream should always be available on a freshly consumed response");
    let mut buf = Vec::new();
    loop {
        match stream.blocking_read(8192) {
            Ok(chunk) if chunk.is_empty() => break,
            Ok(chunk) => buf.extend_from_slice(&chunk),
            Err(e) => {
                return Err(ApiError::new(
                    "http_error",
                    format!("failed to read response body: {e:?}"),
                ))
            }
        }
    }
    drop(stream);
    crate::wasi::http::types::IncomingBody::finish(body);
    Ok(buf)
}

fn send(request: OutgoingRequest) -> Result<Vec<u8>, ApiError> {
    let opts = RequestOptions::new();
    let future = outgoing_handler::handle(request, Some(opts))
        .map_err(|e| ApiError::new("http_error", format!("outgoing-handler: {e:?}")))?;

    future.subscribe().block();

    let response = future
        .get()
        // Unreachable: we blocked until the future resolved above.
        .expect("response must be present after blocking")
        .map_err(|()| ApiError::new("http_error", "response error"))?
        .map_err(|e| ApiError::new("http_error", format!("response: {e:?}")))?;

    let status = response.status();
    let body = response
        .consume()
        // Unreachable: consume() only fails if called more than once.
        .expect("response body should not have been consumed already");
    let bytes = read_body(body)?;

    if status == 200 || status == 201 {
        Ok(bytes)
    } else {
        if let Ok(v) = serde_json::from_slice::<Value>(&bytes) {
            let error = v["error"].as_str().unwrap_or("http_error").to_string();
            let desc = v["error_description"].as_str().map(str::to_string);
            let uri = v["error_uri"].as_str().map(str::to_string);
            Err(ApiError {
                error,
                error_description: desc,
                error_uri: uri,
            })
        } else {
            Err(ApiError::new(
                "http_error",
                format!("HTTP {status}: {}", String::from_utf8_lossy(&bytes)),
            ))
        }
    }
}

fn write_body(req: &OutgoingRequest, body_bytes: &[u8]) -> Result<(), ApiError> {
    let out_body = req
        .body()
        .expect("request body should not have been taken already");
    let writer = out_body
        .write()
        .expect("output stream should always be available on a fresh body");
    writer
        .blocking_write_and_flush(body_bytes)
        .map_err(|e| ApiError::new("http_error", format!("failed to write request body: {e:?}")))?;
    drop(writer);
    OutgoingBody::finish(out_body, None)
        .expect("finishing the body should not fail after writing is complete");
    Ok(())
}

fn post_form_bytes(url: &str, params: &[(&str, &str)]) -> Result<Vec<u8>, ApiError> {
    let parsed = ParsedUrl::parse(url)?;
    let body_bytes = build_query_string(params).into_bytes();

    let headers = Fields::new();
    headers
        .set(
            "Content-Type",
            &[b"application/x-www-form-urlencoded".to_vec()],
        )
        .expect("Content-Type header should be settable on a new Fields");
    headers
        .set(
            "Content-Length",
            &[body_bytes.len().to_string().into_bytes()],
        )
        .expect("Content-Length header should be settable on a new Fields");

    let req = OutgoingRequest::new(headers);
    req.set_method(&Method::Post)
        .expect("method should be settable before the request is sent");
    req.set_scheme(Some(&parsed.scheme))
        .expect("scheme should be settable before the request is sent");
    req.set_authority(Some(&parsed.authority))
        .expect("authority should be settable before the request is sent");
    req.set_path_with_query(Some(&parsed.path_and_query))
        .expect("path should be settable before the request is sent");

    write_body(&req, &body_bytes)?;
    send(req)
}

pub fn post_form(url: &str, params: &[(&str, &str)]) -> Result<Value, ApiError> {
    let bytes = post_form_bytes(url, params)?;
    serde_json::from_slice::<Value>(&bytes).map_err(|e| ApiError::new("parse_error", e.to_string()))
}

pub fn post_form_empty(url: &str, params: &[(&str, &str)]) -> Result<(), ApiError> {
    post_form_bytes(url, params).map(|_| ())
}

pub fn get_json(url: &str, bearer: Option<&str>) -> Result<Value, ApiError> {
    let parsed = ParsedUrl::parse(url)?;

    let headers = Fields::new();
    if let Some(token) = bearer {
        headers
            .set("Authorization", &[format!("Bearer {token}").into_bytes()])
            .expect("Authorization header should be settable on a new Fields");
    }

    let req = OutgoingRequest::new(headers);
    req.set_method(&Method::Get)
        .expect("method should be settable before the request is sent");
    req.set_scheme(Some(&parsed.scheme))
        .expect("scheme should be settable before the request is sent");
    req.set_authority(Some(&parsed.authority))
        .expect("authority should be settable before the request is sent");
    req.set_path_with_query(Some(&parsed.path_and_query))
        .expect("path should be settable before the request is sent");

    let bytes = send(req)?;
    serde_json::from_slice::<Value>(&bytes).map_err(|e| ApiError::new("parse_error", e.to_string()))
}
