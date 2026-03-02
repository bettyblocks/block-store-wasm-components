use serde_json::Value;

use crate::oidc::client::types::{
    DeviceAuthResponse, DiscoveryDocument, Jwk, Jwks, TokenResponse, UserInfo,
};

fn opt_str(v: &Value, key: &str) -> Option<String> {
    v[key].as_str().map(String::from)
}

fn opt_bool(v: &Value, key: &str) -> Option<bool> {
    v[key].as_bool()
}

fn opt_u32(v: &Value, key: &str) -> Option<u32> {
    v[key].as_u64().map(|n| n as u32)
}

fn u32_field(v: &Value, key: &str) -> u32 {
    v[key].as_u64().unwrap_or(0) as u32
}

fn str_list(v: &Value, key: &str) -> Vec<String> {
    v[key]
        .as_array()
        .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------

pub fn json_to_token_response(v: &Value) -> TokenResponse {
    TokenResponse {
        access_token: opt_str(v, "access_token").unwrap_or_default(),
        token_type: opt_str(v, "token_type").unwrap_or_else(|| "Bearer".to_string()),
        expires_in: opt_u32(v, "expires_in"),
        refresh_token: opt_str(v, "refresh_token"),
        scope: opt_str(v, "scope"),
        id_token: opt_str(v, "id_token"),
    }
}

pub fn json_to_device_auth_response(v: &Value) -> DeviceAuthResponse {
    DeviceAuthResponse {
        device_code: opt_str(v, "device_code").unwrap_or_default(),
        user_code: opt_str(v, "user_code").unwrap_or_default(),
        verification_uri: opt_str(v, "verification_uri")
            .or_else(|| opt_str(v, "verification_url"))
            .unwrap_or_default(),
        verification_uri_complete: opt_str(v, "verification_url_complete")
            .or_else(|| opt_str(v, "verification_uri_complete")),
        expires_in: u32_field(v, "expires_in"),
        interval: u32_field(v, "interval"),
    }
}

pub fn json_to_user_info(v: &Value) -> UserInfo {
    UserInfo {
        sub: opt_str(v, "sub").unwrap_or_default(),
        name: opt_str(v, "name"),
        given_name: opt_str(v, "given_name"),
        family_name: opt_str(v, "family_name"),
        picture: opt_str(v, "picture"),
        email: opt_str(v, "email"),
        email_verified: opt_bool(v, "email_verified"),
        locale: opt_str(v, "locale"),
    }
}

pub fn json_to_jwk(v: &Value) -> Jwk {
    Jwk {
        kty: opt_str(v, "kty").unwrap_or_default(),
        key_use: opt_str(v, "use"),
        n: opt_str(v, "n"),
        e: opt_str(v, "e"),
        alg: opt_str(v, "alg"),
        kid: opt_str(v, "kid"),
        x5t: opt_str(v, "x5t"),
        x5c: str_list(v, "x5c"),
    }
}

pub fn json_to_jwks(v: &Value) -> Jwks {
    let keys = v["keys"]
        .as_array()
        .map(|a| a.iter().map(json_to_jwk).collect())
        .unwrap_or_default();
    Jwks { keys }
}

pub fn json_to_discovery(v: &Value) -> DiscoveryDocument {
    DiscoveryDocument {
        issuer: opt_str(v, "issuer").unwrap_or_default(),
        authorization_endpoint: opt_str(v, "authorization_endpoint").unwrap_or_default(),
        device_authorization_endpoint: opt_str(v, "device_authorization_endpoint"),
        token_endpoint: opt_str(v, "token_endpoint").unwrap_or_default(),
        userinfo_endpoint: opt_str(v, "userinfo_endpoint"),
        revocation_endpoint: opt_str(v, "revocation_endpoint"),
        jwks_uri: opt_str(v, "jwks_uri").unwrap_or_default(),
        scopes_supported: str_list(v, "scopes_supported"),
        response_types_supported: str_list(v, "response_types_supported"),
        grant_types_supported: str_list(v, "grant_types_supported"),
        claims_supported: str_list(v, "claims_supported"),
        code_challenge_methods_supported: str_list(v, "code_challenge_methods_supported"),
    }
}
