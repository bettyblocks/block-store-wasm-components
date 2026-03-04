use crate::client::{get_json, post_form, post_form_empty};
use crate::convert::{
    json_to_device_auth_response, json_to_discovery, json_to_jwks, json_to_token_response,
    json_to_user_info,
};
use crate::betty_blocks::open_id_connect::types::{
    ApiError, BearerTokenResult, CodeChallengeMethod, DeviceAuthResponse, DiscoveryDocument, Jwks,
    TokenResponse, UserInfo,
};
use crate::params::build_query_string;

// ---------------------------------------------------------------------------
// Implementations
// ---------------------------------------------------------------------------

/// Pure URL construction — no HTTP.
#[allow(clippy::too_many_arguments)]
pub fn build_authorization_url(
    authorization_endpoint: String,
    client_id: String,
    redirect_uri: String,
    scope: String,
    response_type: String,
    state: Option<String>,
    nonce: Option<String>,
    response_mode: Option<String>,
    code_challenge: Option<String>,
    code_challenge_method: Option<CodeChallengeMethod>,
    login_hint: Option<String>,
    prompt: Option<String>,
) -> Result<String, ApiError> {
    let scope_str = scope
        .split(',')
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(" ");
    let mut params: Vec<(&str, String)> = vec![
        ("client_id", client_id),
        ("redirect_uri", redirect_uri),
        ("scope", scope_str),
        ("response_type", response_type),
    ];

    if let Some(v) = state {
        params.push(("state", v));
    }
    if let Some(v) = nonce {
        params.push(("nonce", v));
    }
    if let Some(v) = response_mode {
        params.push(("response_mode", v));
    }
    if let Some(v) = code_challenge {
        params.push(("code_challenge", v));
    }
    if let Some(m) = code_challenge_method {
        let method = match m {
            CodeChallengeMethod::Plain => "plain".to_string(),
            CodeChallengeMethod::S256 => "S256".to_string(),
        };
        params.push(("code_challenge_method", method));
    }
    if let Some(v) = login_hint {
        params.push(("login_hint", v));
    }
    if let Some(v) = prompt {
        params.push(("prompt", v));
    }

    let pairs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
    Ok(format!(
        "{}?{}",
        authorization_endpoint,
        build_query_string(&pairs)
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
    let verifier_owned;
    if let Some(ref v) = code_verifier {
        verifier_owned = v.clone();
        params.push(("code_verifier", &verifier_owned));
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

pub fn exchange_jwt_bearer(token_endpoint: String, assertion: String) -> BearerTokenResult {
    match post_form(
        &token_endpoint,
        &[
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &assertion),
        ],
    ) {
        Ok(v) => BearerTokenResult {
            response: Some(json_to_token_response(&v)),
            error: None,
        },
        Err(e) => BearerTokenResult {
            response: None,
            error: Some(e),
        },
    }
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
    let secret_owned;
    if let Some(ref s) = client_secret {
        secret_owned = s.clone();
        params.push(("client_secret", &secret_owned));
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
