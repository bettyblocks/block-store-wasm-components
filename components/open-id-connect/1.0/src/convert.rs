use serde::Deserialize;

use crate::betty_blocks::open_id_connect::types::{
    DeviceAuthResponse, DiscoveryDocument, Jwk, Jwks, TokenResponse, UserInfo,
};

fn default_bearer() -> String {
    "Bearer".to_string()
}

#[derive(Deserialize)]
pub struct TokenResponseDe {
    pub access_token: String,
    #[serde(default = "default_bearer")]
    pub token_type: String,
    pub expires_in: Option<u32>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
}

impl From<TokenResponseDe> for TokenResponse {
    fn from(d: TokenResponseDe) -> Self {
        TokenResponse {
            access_token: d.access_token,
            token_type: d.token_type,
            expires_in: d.expires_in,
            refresh_token: d.refresh_token,
            scope: d.scope,
            id_token: d.id_token,
        }
    }
}

#[derive(Deserialize)]
pub struct DeviceAuthResponseDe {
    pub device_code: String,
    pub user_code: String,
    /// Accepts either the RFC 8628 name (`verification_uri`) or the older
    /// Google-style alias (`verification_url`).
    #[serde(alias = "verification_url")]
    pub verification_uri: String,
    #[serde(alias = "verification_url_complete")]
    pub verification_uri_complete: Option<String>,
    pub expires_in: u32,
    #[serde(default)]
    pub interval: u32,
}

impl From<DeviceAuthResponseDe> for DeviceAuthResponse {
    fn from(d: DeviceAuthResponseDe) -> Self {
        DeviceAuthResponse {
            device_code: d.device_code,
            user_code: d.user_code,
            verification_uri: d.verification_uri,
            verification_uri_complete: d.verification_uri_complete,
            expires_in: d.expires_in,
            interval: d.interval,
        }
    }
}

#[derive(Deserialize)]
pub struct UserInfoDe {
    pub sub: String,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub locale: Option<String>,
}

impl From<UserInfoDe> for UserInfo {
    fn from(d: UserInfoDe) -> Self {
        UserInfo {
            sub: d.sub,
            name: d.name,
            given_name: d.given_name,
            family_name: d.family_name,
            picture: d.picture,
            email: d.email,
            email_verified: d.email_verified,
            locale: d.locale,
        }
    }
}

#[derive(Deserialize)]
pub struct JwkDe {
    pub kty: String,
    #[serde(rename = "use")]
    pub key_use: Option<String>,
    pub n: Option<String>,
    pub e: Option<String>,
    pub alg: Option<String>,
    pub kid: Option<String>,
    pub x5t: Option<String>,
    #[serde(default)]
    pub x5c: Vec<String>,
}

impl From<JwkDe> for Jwk {
    fn from(d: JwkDe) -> Self {
        Jwk {
            kty: d.kty,
            key_use: d.key_use,
            n: d.n,
            e: d.e,
            alg: d.alg,
            kid: d.kid,
            x5t: d.x5t,
            x5c: d.x5c,
        }
    }
}

#[derive(Deserialize)]
pub struct JwksDe {
    pub keys: Vec<JwkDe>,
}

impl From<JwksDe> for Jwks {
    fn from(d: JwksDe) -> Self {
        Jwks {
            keys: d.keys.into_iter().map(Jwk::from).collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct DiscoveryDocumentDe {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub device_authorization_endpoint: Option<String>,
    pub token_endpoint: String,
    pub userinfo_endpoint: Option<String>,
    pub revocation_endpoint: Option<String>,
    pub jwks_uri: String,
    #[serde(default)]
    pub scopes_supported: Vec<String>,
    #[serde(default)]
    pub response_types_supported: Vec<String>,
    #[serde(default)]
    pub grant_types_supported: Vec<String>,
    #[serde(default)]
    pub claims_supported: Vec<String>,
    #[serde(default)]
    pub code_challenge_methods_supported: Vec<String>,
}

impl From<DiscoveryDocumentDe> for DiscoveryDocument {
    fn from(d: DiscoveryDocumentDe) -> Self {
        DiscoveryDocument {
            issuer: d.issuer,
            authorization_endpoint: d.authorization_endpoint,
            device_authorization_endpoint: d.device_authorization_endpoint,
            token_endpoint: d.token_endpoint,
            userinfo_endpoint: d.userinfo_endpoint,
            revocation_endpoint: d.revocation_endpoint,
            jwks_uri: d.jwks_uri,
            scopes_supported: d.scopes_supported,
            response_types_supported: d.response_types_supported,
            grant_types_supported: d.grant_types_supported,
            claims_supported: d.claims_supported,
            code_challenge_methods_supported: d.code_challenge_methods_supported,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
        let v =
            json!({ "access_token": "", "token_type": "Bearer", "id_token": "eyJ.payload.sig" });
        assert_eq!(parse_token(v).id_token.as_deref(), Some("eyJ.payload.sig"));
    }

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
        assert!(
            d.grant_types_supported
                .contains(&"authorization_code".to_string())
        );
        assert!(
            d.grant_types_supported
                .contains(&"refresh_token".to_string())
        );
    }

    #[test]
    fn discovery_parses_code_challenge_methods() {
        let v = json!({
            "issuer": "", "authorization_endpoint": "", "token_endpoint": "", "jwks_uri": "",
            "code_challenge_methods_supported": ["S256", "plain"]
        });
        let d = parse_discovery(v);
        assert!(
            d.code_challenge_methods_supported
                .contains(&"S256".to_string())
        );
        assert!(
            d.code_challenge_methods_supported
                .contains(&"plain".to_string())
        );
    }

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
}
