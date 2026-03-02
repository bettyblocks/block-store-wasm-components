use serde_json::json;

use crate::convert::{
    json_to_device_auth_response, json_to_discovery, json_to_jwks, json_to_token_response,
    json_to_user_info,
};

// -----------------------------------------------------------------------
// json_to_token_response
// -----------------------------------------------------------------------

#[test]
fn token_response_parses_access_token() {
    let v = json!({ "access_token": "abc123", "token_type": "Bearer" });
    assert_eq!(json_to_token_response(&v).access_token, "abc123");
}

#[test]
fn token_response_parses_token_type() {
    let v = json!({ "access_token": "", "token_type": "MAC" });
    assert_eq!(json_to_token_response(&v).token_type, "MAC");
}

#[test]
fn token_response_defaults_token_type_to_bearer_when_absent() {
    let v = json!({ "access_token": "x" });
    assert_eq!(json_to_token_response(&v).token_type, "Bearer");
}

#[test]
fn token_response_parses_expires_in() {
    let v = json!({ "access_token": "", "token_type": "Bearer", "expires_in": 3600 });
    assert_eq!(json_to_token_response(&v).expires_in, Some(3600));
}

#[test]
fn token_response_expires_in_absent_is_none() {
    let v = json!({ "access_token": "", "token_type": "Bearer" });
    assert!(json_to_token_response(&v).expires_in.is_none());
}

#[test]
fn token_response_refresh_token_present() {
    let v = json!({ "access_token": "", "token_type": "Bearer", "refresh_token": "rt123" });
    assert_eq!(json_to_token_response(&v).refresh_token.as_deref(), Some("rt123"));
}

#[test]
fn token_response_refresh_token_absent_is_none() {
    let v = json!({ "access_token": "", "token_type": "Bearer" });
    assert!(json_to_token_response(&v).refresh_token.is_none());
}

#[test]
fn token_response_parses_id_token() {
    let v = json!({ "access_token": "", "token_type": "Bearer", "id_token": "eyJ.payload.sig" });
    assert_eq!(json_to_token_response(&v).id_token.as_deref(), Some("eyJ.payload.sig"));
}

// -----------------------------------------------------------------------
// json_to_device_auth_response
// -----------------------------------------------------------------------

#[test]
fn device_auth_response_parses_required_fields() {
    let v = json!({
        "device_code": "dc123",
        "user_code": "ABCD-1234",
        "verification_uri": "https://example.com/activate",
        "expires_in": 1800,
        "interval": 5
    });
    let r = json_to_device_auth_response(&v);
    assert_eq!(r.device_code, "dc123");
    assert_eq!(r.user_code, "ABCD-1234");
    assert_eq!(r.verification_uri, "https://example.com/activate");
    assert_eq!(r.expires_in, 1800);
    assert_eq!(r.interval, 5);
}

#[test]
fn device_auth_response_falls_back_to_verification_url() {
    let v = json!({
        "device_code": "", "user_code": "",
        "verification_url": "https://example.com/activate",
        "expires_in": 0, "interval": 0
    });
    assert_eq!(
        json_to_device_auth_response(&v).verification_uri,
        "https://example.com/activate"
    );
}

#[test]
fn device_auth_response_prefers_verification_uri_over_url() {
    let v = json!({
        "device_code": "", "user_code": "",
        "verification_uri": "https://example.com/uri",
        "verification_url": "https://example.com/url",
        "expires_in": 0, "interval": 0
    });
    assert_eq!(
        json_to_device_auth_response(&v).verification_uri,
        "https://example.com/uri"
    );
}

// -----------------------------------------------------------------------
// json_to_discovery
// -----------------------------------------------------------------------

#[test]
fn discovery_parses_required_fields() {
    let v = json!({
        "issuer": "https://example.com",
        "authorization_endpoint": "https://example.com/auth",
        "token_endpoint": "https://example.com/token",
        "jwks_uri": "https://example.com/jwks"
    });
    let d = json_to_discovery(&v);
    assert_eq!(d.issuer, "https://example.com");
    assert_eq!(d.authorization_endpoint, "https://example.com/auth");
    assert_eq!(d.token_endpoint, "https://example.com/token");
    assert_eq!(d.jwks_uri, "https://example.com/jwks");
}

#[test]
fn discovery_optional_fields_absent_when_missing() {
    let v = json!({
        "issuer": "", "authorization_endpoint": "",
        "token_endpoint": "", "jwks_uri": ""
    });
    let d = json_to_discovery(&v);
    assert!(d.device_authorization_endpoint.is_none());
    assert!(d.userinfo_endpoint.is_none());
    assert!(d.revocation_endpoint.is_none());
}

#[test]
fn discovery_parses_scopes_supported() {
    let v = json!({
        "issuer": "", "authorization_endpoint": "", "token_endpoint": "", "jwks_uri": "",
        "scopes_supported": ["openid", "email", "profile"]
    });
    assert_eq!(
        json_to_discovery(&v).scopes_supported,
        vec!["openid", "email", "profile"]
    );
}

#[test]
fn discovery_parses_grant_types_supported() {
    let v = json!({
        "issuer": "", "authorization_endpoint": "", "token_endpoint": "", "jwks_uri": "",
        "grant_types_supported": ["authorization_code", "refresh_token"]
    });
    let d = json_to_discovery(&v);
    assert!(d.grant_types_supported.contains(&"authorization_code".to_string()));
    assert!(d.grant_types_supported.contains(&"refresh_token".to_string()));
}

#[test]
fn discovery_parses_code_challenge_methods() {
    let v = json!({
        "issuer": "", "authorization_endpoint": "", "token_endpoint": "", "jwks_uri": "",
        "code_challenge_methods_supported": ["S256", "plain"]
    });
    let d = json_to_discovery(&v);
    assert!(d.code_challenge_methods_supported.contains(&"S256".to_string()));
    assert!(d.code_challenge_methods_supported.contains(&"plain".to_string()));
}

// -----------------------------------------------------------------------
// json_to_jwks
// -----------------------------------------------------------------------

#[test]
fn jwks_parses_single_key() {
    let v = json!({ "keys": [{ "kty": "RSA", "n": "abc", "e": "AQAB", "kid": "key1" }] });
    let jwks = json_to_jwks(&v);
    assert_eq!(jwks.keys.len(), 1);
    assert_eq!(jwks.keys[0].kty, "RSA");
    assert_eq!(jwks.keys[0].kid.as_deref(), Some("key1"));
}

#[test]
fn jwks_empty_keys_array() {
    let v = json!({ "keys": [] });
    assert_eq!(json_to_jwks(&v).keys.len(), 0);
}

// -----------------------------------------------------------------------
// json_to_user_info
// -----------------------------------------------------------------------

#[test]
fn user_info_parses_sub() {
    let v = json!({ "sub": "user123" });
    assert_eq!(json_to_user_info(&v).sub, "user123");
}

#[test]
fn user_info_optional_fields_absent_when_missing() {
    let v = json!({ "sub": "user123" });
    let u = json_to_user_info(&v);
    assert!(u.name.is_none());
    assert!(u.email.is_none());
    assert!(u.email_verified.is_none());
}

#[test]
fn user_info_parses_email_and_verified() {
    let v = json!({ "sub": "x", "email": "test@example.com", "email_verified": true });
    let u = json_to_user_info(&v);
    assert_eq!(u.email.as_deref(), Some("test@example.com"));
    assert_eq!(u.email_verified, Some(true));
}

#[test]
fn user_info_parses_name_fields() {
    let v = json!({
        "sub": "x",
        "name": "Jane Doe",
        "given_name": "Jane",
        "family_name": "Doe"
    });
    let u = json_to_user_info(&v);
    assert_eq!(u.name.as_deref(), Some("Jane Doe"));
    assert_eq!(u.given_name.as_deref(), Some("Jane"));
    assert_eq!(u.family_name.as_deref(), Some("Doe"));
}
