use serde::Deserialize;

use crate::betty_blocks::open_id_connect::types::{
    DeviceAuthResponse, DiscoveryDocument, Jwk, Jwks, TokenResponse, UserInfo,
};

fn default_bearer() -> String {
    "Bearer".to_string()
}

// ---------------------------------------------------------------------------
// TokenResponse
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// DeviceAuthResponse
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// UserInfo
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Jwk / Jwks
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// DiscoveryDocument
// ---------------------------------------------------------------------------

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
