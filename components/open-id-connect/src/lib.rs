wit_bindgen::generate!({
    world: "oidc-client-world",
    path: "wit",
    generate_all,
});

mod auth;
mod client;
mod convert;
mod params;

#[cfg(test)]
mod tests;

use exports::oidc::client::oidc_client::Guest;
use oidc::client::types::{
    ApiError, BearerTokenResult, CodeChallengeMethod, DeviceAuthResponse, DiscoveryDocument, Jwks,
    TokenResponse, UserInfo,
};

struct Component;

export!(Component);

impl Guest for Component {
    fn build_authorization_url(
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
        auth::build_authorization_url(
            authorization_endpoint,
            client_id,
            redirect_uri,
            scope,
            response_type,
            state,
            nonce,
            response_mode,
            code_challenge,
            code_challenge_method,
            login_hint,
            prompt,
        )
    }

    fn exchange_code(
        token_endpoint: String,
        client_id: String,
        client_secret: String,
        code: String,
        redirect_uri: String,
        code_verifier: Option<String>,
    ) -> Result<TokenResponse, ApiError> {
        auth::exchange_code(
            token_endpoint,
            client_id,
            client_secret,
            code,
            redirect_uri,
            code_verifier,
        )
    }

    fn refresh_access_token(
        token_endpoint: String,
        client_id: String,
        client_secret: String,
        refresh_token: String,
    ) -> Result<TokenResponse, ApiError> {
        auth::refresh_access_token(token_endpoint, client_id, client_secret, refresh_token)
    }

    fn exchange_jwt_bearer(token_endpoint: String, assertion: String) -> BearerTokenResult {
        auth::exchange_jwt_bearer(token_endpoint, assertion)
    }

    fn initiate_device_auth(
        device_authorization_endpoint: String,
        client_id: String,
        scope: String,
    ) -> Result<DeviceAuthResponse, ApiError> {
        auth::initiate_device_auth(device_authorization_endpoint, client_id, scope)
    }

    fn poll_device_token(
        token_endpoint: String,
        client_id: String,
        client_secret: Option<String>,
        device_code: String,
    ) -> Result<TokenResponse, ApiError> {
        auth::poll_device_token(token_endpoint, client_id, client_secret, device_code)
    }

    fn get_userinfo(userinfo_endpoint: String, access_token: String) -> Result<UserInfo, ApiError> {
        auth::get_userinfo(userinfo_endpoint, access_token)
    }

    fn revoke_token(revocation_endpoint: String, token: String) -> Result<(), ApiError> {
        auth::revoke_token(revocation_endpoint, token)
    }

    fn get_jwks(jwks_uri: String) -> Result<Jwks, ApiError> {
        auth::get_jwks(jwks_uri)
    }

    fn get_discovery(url: String) -> Result<DiscoveryDocument, ApiError> {
        auth::get_discovery(url)
    }
}
