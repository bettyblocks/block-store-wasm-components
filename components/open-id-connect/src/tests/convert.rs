use serde_json::json;

use crate::convert::{
    DeviceAuthResponseDe, DiscoveryDocumentDe, JwksDe, TokenResponseDe, UserInfoDe,
};

fn parse_token(v: serde_json::Value) -> TokenResponseDe {
    serde_json::from_value(v).unwrap()
}

fn parse_device(v: serde_json::Value) -> DeviceAuthResponseDe {
    serde_json::from_value(v).unwrap()
}

fn parse_discovery(v: serde_json::Value) -> DiscoveryDocumentDe {
    serde_json::from_value(v).unwrap()
}

fn parse_jwks(v: serde_json::Value) -> JwksDe {
    serde_json::from_value(v).unwrap()
}

fn parse_user_info(v: serde_json::Value) -> UserInfoDe {
    serde_json::from_value(v).unwrap()
}

// -----------------------------------------------------------------------
// TokenResponseDe
// -----------------------------------------------------------------------

#[test]
fn token_response_parses_access_token() {
    let v = json!({ "access_token": "abc123", "token_type": "Bearer" });
    assert_eq!(parse_token(v).access_token, "abc123");
}

#[test]
fn token_response_parses_token_type() {
    let v = json!({ "access_token": "", "token_type": "MAC" });
    assert_eq!(parse_token(v).token_type, "MAC");
}

#[test]
fn token_response_defaults_token_type_to_bearer_when_absent() {
    let v = json!({ "access_token": "x" });
    assert_eq!(parse_token(v).token_type, "Bearer");
}

#[test]
fn token_response_parses_expires_in() {
    let v = json!({ "access_token": "", "token_type": "Bearer", "expires_in": 3600 });
    assert_eq!(parse_token(v).expires_in, Some(3600));
}

#[test]
fn token_response_expires_in_absent_is_none() {
    let v = json!({ "access_token": "", "token_type": "Bearer" });
    assert!(parse_token(v).expires_in.is_none());
}

#[test]
fn token_response_refresh_token_present() {
    let v = json!({ "access_token": "", "token_type": "Bearer", "refresh_token": "rt123" });
    assert_eq!(parse_token(v).refresh_token.as_deref(), Some("rt123"));
}

#[test]
fn token_response_refresh_token_absent_is_none() {
    let v = json!({ "access_token": "", "token_type": "Bearer" });
    assert!(parse_token(v).refresh_token.is_none());
}

#[test]
fn token_response_parses_id_token() {
    let v = json!({ "access_token": "", "token_type": "Bearer", "id_token": "eyJ.payload.sig" });
    assert_eq!(parse_token(v).id_token.as_deref(), Some("eyJ.payload.sig"));
}

// -----------------------------------------------------------------------
// DeviceAuthResponseDe
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
    let r = parse_device(v);
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
        parse_device(v).verification_uri,
        "https://example.com/activate"
    );
}

// -----------------------------------------------------------------------
// DiscoveryDocumentDe
// -----------------------------------------------------------------------

#[test]
fn discovery_parses_required_fields() {
    let v = json!({
        "issuer": "https://example.com",
        "authorization_endpoint": "https://example.com/auth",
        "token_endpoint": "https://example.com/token",
        "jwks_uri": "https://example.com/jwks"
    });
    let d = parse_discovery(v);
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
    let d = parse_discovery(v);
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
        parse_discovery(v).scopes_supported,
        vec!["openid", "email", "profile"]
    );
}

#[test]
fn discovery_parses_grant_types_supported() {
    let v = json!({
        "issuer": "", "authorization_endpoint": "", "token_endpoint": "", "jwks_uri": "",
        "grant_types_supported": ["authorization_code", "refresh_token"]
    });
    let d = parse_discovery(v);
    assert!(d
        .grant_types_supported
        .contains(&"authorization_code".to_string()));
    assert!(d
        .grant_types_supported
        .contains(&"refresh_token".to_string()));
}

#[test]
fn discovery_parses_code_challenge_methods() {
    let v = json!({
        "issuer": "", "authorization_endpoint": "", "token_endpoint": "", "jwks_uri": "",
        "code_challenge_methods_supported": ["S256", "plain"]
    });
    let d = parse_discovery(v);
    assert!(d
        .code_challenge_methods_supported
        .contains(&"S256".to_string()));
    assert!(d
        .code_challenge_methods_supported
        .contains(&"plain".to_string()));
}

// -----------------------------------------------------------------------
// JwksDe
// -----------------------------------------------------------------------

#[test]
fn jwks_parses_single_key() {
    let v = json!({ "keys": [{ "kty": "RSA", "n": "abc", "e": "AQAB", "kid": "key1" }] });
    let jwks = parse_jwks(v);
    assert_eq!(jwks.keys.len(), 1);
    assert_eq!(jwks.keys[0].kty, "RSA");
    assert_eq!(jwks.keys[0].kid.as_deref(), Some("key1"));
}

#[test]
fn jwks_empty_keys_array() {
    let v = json!({ "keys": [] });
    assert_eq!(parse_jwks(v).keys.len(), 0);
}

// -----------------------------------------------------------------------
// UserInfoDe
// -----------------------------------------------------------------------

#[test]
fn user_info_parses_sub() {
    let v = json!({ "sub": "user123" });
    assert_eq!(parse_user_info(v).sub, "user123");
}

#[test]
fn user_info_optional_fields_absent_when_missing() {
    let v = json!({ "sub": "user123" });
    let u = parse_user_info(v);
    assert!(u.name.is_none());
    assert!(u.email.is_none());
    assert!(u.email_verified.is_none());
}

#[test]
fn user_info_parses_email_and_verified() {
    let v = json!({ "sub": "x", "email": "test@example.com", "email_verified": true });
    let u = parse_user_info(v);
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
    let u = parse_user_info(v);
    assert_eq!(u.name.as_deref(), Some("Jane Doe"));
    assert_eq!(u.given_name.as_deref(), Some("Jane"));
    assert_eq!(u.family_name.as_deref(), Some("Doe"));
}
