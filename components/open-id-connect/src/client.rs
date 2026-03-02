use serde_json::Value;

use crate::oidc::client::types::ApiError;
use crate::params::build_query_string;

// ---------------------------------------------------------------------------
// WASI HTTP imports
// ---------------------------------------------------------------------------

use crate::wasi::http::outgoing_handler;
use crate::wasi::http::types::{
    Fields, Method, OutgoingBody, OutgoingRequest, RequestOptions, Scheme,
};

// ---------------------------------------------------------------------------
// Error helper
// ---------------------------------------------------------------------------

fn api_err(error: &str, description: &str) -> ApiError {
    ApiError {
        error: error.to_string(),
        error_description: Some(description.to_string()),
        error_uri: None,
    }
}

// ---------------------------------------------------------------------------
// URL parsing
// ---------------------------------------------------------------------------

struct ParsedUrl {
    scheme: Scheme,
    authority: String,
    path_and_query: String,
}

fn parse_url(url: &str) -> Result<ParsedUrl, ApiError> {
    let (scheme, rest) = if let Some(r) = url.strip_prefix("https://") {
        (Scheme::Https, r)
    } else if let Some(r) = url.strip_prefix("http://") {
        (Scheme::Http, r)
    } else {
        return Err(ApiError {
            error: "invalid_url".to_string(),
            error_description: Some(format!("Unsupported scheme in URL: {url}")),
            error_uri: None,
        });
    };

    let (authority, path_and_query) = match rest.find('/') {
        Some(i) => (rest[..i].to_string(), rest[i..].to_string()),
        None => (rest.to_string(), "/".to_string()),
    };

    Ok(ParsedUrl { scheme, authority, path_and_query })
}

// ---------------------------------------------------------------------------
// WASI HTTP helpers
// ---------------------------------------------------------------------------

fn read_body(body: crate::wasi::http::types::IncomingBody) -> Vec<u8> {
    let stream = body.stream().unwrap();
    let mut buf = Vec::new();
    loop {
        match stream.blocking_read(8192) {
            Ok(chunk) if chunk.is_empty() => break,
            Ok(chunk) => buf.extend_from_slice(&chunk),
            Err(_) => break,
        }
    }
    drop(stream);
    crate::wasi::http::types::IncomingBody::finish(body);
    buf
}

fn send(request: OutgoingRequest) -> Result<Vec<u8>, ApiError> {
    let opts = RequestOptions::new();
    let future = outgoing_handler::handle(request, Some(opts))
        .map_err(|e| api_err("http_error", &format!("outgoing-handler: {e:?}")))?;

    future.subscribe().block();

    let response = future
        .get()
        .ok_or_else(|| api_err("http_error", "no response"))?
        .map_err(|()| api_err("http_error", "response error"))?
        .map_err(|e| api_err("http_error", &format!("response: {e:?}")))?;

    let status = response.status();
    let body = response.consume().unwrap();
    let bytes = read_body(body);

    if status == 200 || status == 201 {
        Ok(bytes)
    } else {
        // Try to parse a JSON error body
        if let Ok(v) = serde_json::from_slice::<Value>(&bytes) {
            let error = v["error"].as_str().unwrap_or("http_error").to_string();
            let desc = v["error_description"].as_str().map(str::to_string);
            let uri = v["error_uri"].as_str().map(str::to_string);
            Err(ApiError { error, error_description: desc, error_uri: uri })
        } else {
            Err(api_err(
                "http_error",
                &format!("HTTP {status}: {}", String::from_utf8_lossy(&bytes)),
            ))
        }
    }
}

// ---------------------------------------------------------------------------
// Public client API
// ---------------------------------------------------------------------------

/// POST `application/x-www-form-urlencoded` → parse JSON response.
pub fn post_form(url: &str, params: &[(&str, &str)]) -> Result<Value, ApiError> {
    let parsed = parse_url(url)?;
    let body_bytes = build_query_string(params).into_bytes();

    let headers = Fields::new();
    headers
        .set(
            &"Content-Type".to_string(),
            &[b"application/x-www-form-urlencoded".to_vec()],
        )
        .ok();
    headers
        .set(
            &"Content-Length".to_string(),
            &[body_bytes.len().to_string().into_bytes()],
        )
        .ok();

    let req = OutgoingRequest::new(headers);
    req.set_method(&Method::Post).ok();
    req.set_scheme(Some(&parsed.scheme)).ok();
    req.set_authority(Some(&parsed.authority)).ok();
    req.set_path_with_query(Some(&parsed.path_and_query)).ok();

    let out_body = req.body().unwrap();
    let writer = out_body.write().unwrap();
    writer.blocking_write_and_flush(&body_bytes).ok();
    drop(writer);
    OutgoingBody::finish(out_body, None).ok();

    let bytes = send(req)?;
    serde_json::from_slice::<Value>(&bytes)
        .map_err(|e| api_err("parse_error", &e.to_string()))
}

/// POST `application/x-www-form-urlencoded` → expect empty / 200 body.
pub fn post_form_empty(url: &str, params: &[(&str, &str)]) -> Result<(), ApiError> {
    let parsed = parse_url(url)?;
    let body_bytes = build_query_string(params).into_bytes();

    let headers = Fields::new();
    headers
        .set(
            &"Content-Type".to_string(),
            &[b"application/x-www-form-urlencoded".to_vec()],
        )
        .ok();

    let req = OutgoingRequest::new(headers);
    req.set_method(&Method::Post).ok();
    req.set_scheme(Some(&parsed.scheme)).ok();
    req.set_authority(Some(&parsed.authority)).ok();
    req.set_path_with_query(Some(&parsed.path_and_query)).ok();

    let out_body = req.body().unwrap();
    let writer = out_body.write().unwrap();
    writer.blocking_write_and_flush(&body_bytes).ok();
    drop(writer);
    OutgoingBody::finish(out_body, None).ok();

    send(req).map(|_| ())
}

/// GET with an optional `Authorization: Bearer` header → parse JSON response.
pub fn get_json(url: &str, bearer: Option<&str>) -> Result<Value, ApiError> {
    let parsed = parse_url(url)?;

    let headers = Fields::new();
    if let Some(token) = bearer {
        headers
            .set(
                &"Authorization".to_string(),
                &[format!("Bearer {token}").into_bytes()],
            )
            .ok();
    }

    let req = OutgoingRequest::new(headers);
    req.set_method(&Method::Get).ok();
    req.set_scheme(Some(&parsed.scheme)).ok();
    req.set_authority(Some(&parsed.authority)).ok();
    req.set_path_with_query(Some(&parsed.path_and_query)).ok();

    let bytes = send(req)?;
    serde_json::from_slice::<Value>(&bytes)
        .map_err(|e| api_err("parse_error", &e.to_string()))
}
