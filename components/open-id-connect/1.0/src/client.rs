use crate::betty_blocks::open_id_connect::types::{ApiError, ServerErrorBody};
use crate::params::build_query_string;
use wstd::http::{Body, Client, Request};
use wstd::runtime::block_on;

/// Deserialization target for RFC 6749 §5.2 error responses.
#[derive(serde::Deserialize)]
struct OAuthError {
    #[serde(default = "default_server_error")]
    error: String,
    error_description: Option<String>,
    error_uri: Option<String>,
}

fn default_server_error() -> String {
    "server_error".to_string()
}

async fn send<T: for<'de> serde::Deserialize<'de>>(request: Request<Body>) -> Result<T, ApiError> {
    let response = Client::new()
        .send(request)
        .await
        .map_err(|err| ApiError::HttpError(err.to_string()))?;

    let status = response.status();
    let mut body = response.into_body();

    if status.is_success() {
        body.json::<T>()
            .await
            .map_err(|err| ApiError::ParseError(err.to_string()))
    } else if let Ok(err) = body.json::<OAuthError>().await {
        Err(ApiError::ServerError(ServerErrorBody {
            error: err.error,
            error_description: err.error_description,
            error_uri: err.error_uri,
        }))
    } else {
        Err(ApiError::HttpError(format!(
            "HTTP {status}: {}",
            body.str_contents()
                .await
                .unwrap_or("Could not read response body")
        )))
    }
}

fn build_form_request(url: &str, params: &[(&str, &str)]) -> Result<Request<Body>, ApiError> {
    let body_string = build_query_string(params);
    let content_length = body_string.len().to_string();
    Request::post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Content-Length", &content_length)
        .body(Body::from(body_string.as_bytes()))
        .map_err(|err| ApiError::InvalidUrl(err.to_string()))
}

pub fn post_form<T: for<'de> serde::Deserialize<'de>>(
    url: &str,
    params: &[(&str, &str)],
) -> Result<T, ApiError> {
    let request = build_form_request(url, params)?;
    block_on(send(request))
}

pub fn post_form_empty(url: &str, params: &[(&str, &str)]) -> Result<(), ApiError> {
    let request = build_form_request(url, params)?;
    block_on(send::<serde_json::Value>(request))?;
    Ok(())
}

pub fn get_json<T: for<'de> serde::Deserialize<'de>>(
    url: &str,
    bearer: Option<&str>,
) -> Result<T, ApiError> {
    let mut builder = Request::get(url);
    if let Some(token) = bearer {
        builder = builder.header("Authorization", &format!("Bearer {token}"));
    }
    let request = builder
        .body(Body::empty())
        .map_err(|err| ApiError::InvalidUrl(err.to_string()))?;
    block_on(send(request))
}
