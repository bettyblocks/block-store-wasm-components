use crate::betty_blocks::open_id_connect::types::{
    ApiError, CodeChallengeMethod, DeviceAuthResponse, DiscoveryDocument, Jwks, TokenResponse,
    UserInfo,
};
use crate::client::{get_json, post_form, post_form_empty};
use crate::convert::{
    json_to_device_auth_response, json_to_discovery, json_to_jwks, json_to_token_response,
    json_to_user_info,
};
use crate::params::build_query_string;

pub struct AuthorizationUrlOptions {
    pub state: Option<String>,
    pub nonce: Option<String>,
    pub response_mode: Option<String>,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<CodeChallengeMethod>,
    pub login_hint: Option<String>,
    pub prompt: Option<String>,
}

pub fn build_authorization_url(
    authorization_endpoint: String,
    client_id: String,
    redirect_uri: String,
    scope: String,
    response_type: String,
    options: AuthorizationUrlOptions,
) -> Result<String, ApiError> {
    let scope_str = scope
        .split(',')
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(" ");

    let mut params: Vec<(&str, &str)> = vec![
        ("client_id", &client_id),
        ("redirect_uri", &redirect_uri),
        ("scope", &scope_str),
        ("response_type", &response_type),
    ];

    if let Some(ref v) = options.state {
        params.push(("state", v));
    }
    if let Some(ref v) = options.nonce {
        params.push(("nonce", v));
    }
    if let Some(ref v) = options.response_mode {
        params.push(("response_mode", v));
    }
    if let Some(ref v) = options.code_challenge {
        params.push(("code_challenge", v));
    }
    match options.code_challenge_method {
        Some(CodeChallengeMethod::Plain) => params.push(("code_challenge_method", "plain")),
        Some(CodeChallengeMethod::S256) => params.push(("code_challenge_method", "S256")),
        None => {}
    }
    if let Some(ref v) = options.login_hint {
        params.push(("login_hint", v));
    }
    if let Some(ref v) = options.prompt {
        params.push(("prompt", v));
    }

    Ok(format!(
        "{}?{}",
        authorization_endpoint,
        build_query_string(&params)
    ))
}

pub fn exchange_code(
    token_endpoint: String,
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
    code_verifier: Option<String>,
) -> Result<TokenResponse, ApiError> {
    let mut params = vec![
        ("grant_type", "authorization_code"),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
        ("code", &code),
        ("redirect_uri", &redirect_uri),
    ];
    if let Some(ref v) = code_verifier {
        params.push(("code_verifier", v));
    }
    let v = post_form(&token_endpoint, &params)?;
    Ok(json_to_token_response(&v))
}

pub fn refresh_access_token(
    token_endpoint: String,
    client_id: String,
    client_secret: String,
    refresh_token: String,
) -> Result<TokenResponse, ApiError> {
    let v = post_form(
        &token_endpoint,
        &[
            ("grant_type", "refresh_token"),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
            ("refresh_token", &refresh_token),
        ],
    )?;
    Ok(json_to_token_response(&v))
}

pub fn exchange_jwt_bearer(
    token_endpoint: String,
    assertion: String,
) -> Result<TokenResponse, ApiError> {
    let v = post_form(
        &token_endpoint,
        &[
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &assertion),
        ],
    )?;
    Ok(json_to_token_response(&v))
}

pub fn initiate_device_auth(
    device_authorization_endpoint: String,
    client_id: String,
    scope: String,
) -> Result<DeviceAuthResponse, ApiError> {
    let scope_str = scope
        .split(',')
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(" ");
    let v = post_form(
        &device_authorization_endpoint,
        &[("client_id", &client_id), ("scope", &scope_str)],
    )?;
    Ok(json_to_device_auth_response(&v))
}

pub fn poll_device_token(
    token_endpoint: String,
    client_id: String,
    client_secret: Option<String>,
    device_code: String,
) -> Result<TokenResponse, ApiError> {
    let mut params = vec![
        ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
        ("client_id", &client_id),
        ("device_code", &device_code),
    ];
    if let Some(ref s) = client_secret {
        params.push(("client_secret", s));
    }
    let v = post_form(&token_endpoint, &params)?;
    Ok(json_to_token_response(&v))
}

pub fn get_userinfo(userinfo_endpoint: String, access_token: String) -> Result<UserInfo, ApiError> {
    let v = get_json(&userinfo_endpoint, Some(&access_token))?;
    Ok(json_to_user_info(&v))
}

pub fn revoke_token(revocation_endpoint: String, token: String) -> Result<(), ApiError> {
    post_form_empty(&revocation_endpoint, &[("token", &token)])
}

pub fn get_jwks(jwks_uri: String) -> Result<Jwks, ApiError> {
    let v = get_json(&jwks_uri, None)?;
    Ok(json_to_jwks(&v))
}

pub fn get_discovery(url: String) -> Result<DiscoveryDocument, ApiError> {
    let v = get_json(&url, None)?;
    Ok(json_to_discovery(&v))
}
