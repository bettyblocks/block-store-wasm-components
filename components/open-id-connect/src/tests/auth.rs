use crate::auth::build_authorization_url;
use crate::betty_blocks::open_id_connect::types::CodeChallengeMethod;

fn build_url(
    endpoint: &str,
    client_id: &str,
    redirect_uri: &str,
    scope: &str,
    response_type: &str,
) -> String {
    build_authorization_url(
        endpoint.into(),
        client_id.into(),
        redirect_uri.into(),
        scope.into(),
        response_type.into(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap()
}

// -----------------------------------------------------------------------
// Required params
// -----------------------------------------------------------------------

#[test]
fn build_authorization_url_contains_base_endpoint() {
    let url = build_url(
        "https://example.com/auth",
        "client1",
        "https://app/cb",
        "openid",
        "code",
    );
    assert!(url.starts_with("https://example.com/auth?"));
}

#[test]
fn build_authorization_url_includes_client_id() {
    let url = build_url(
        "https://example.com/auth",
        "my-client",
        "https://app/cb",
        "openid",
        "code",
    );
    assert!(url.contains("client_id=my-client"), "url: {url}");
}

#[test]
fn build_authorization_url_includes_redirect_uri() {
    let url = build_url(
        "https://example.com/auth",
        "c",
        "https://app/callback",
        "openid",
        "code",
    );
    assert!(
        url.contains("redirect_uri=https%3A%2F%2Fapp%2Fcallback"),
        "url: {url}"
    );
}

#[test]
fn build_authorization_url_includes_response_type() {
    let url = build_url(
        "https://example.com/auth",
        "c",
        "https://app/cb",
        "openid",
        "code",
    );
    assert!(url.contains("response_type=code"), "url: {url}");
}

#[test]
fn build_authorization_url_converts_comma_scope_to_space_separated() {
    let url = build_url(
        "https://example.com/auth",
        "c",
        "https://app/cb",
        "openid,email,profile",
        "code",
    );
    assert!(url.contains("scope=openid%20email%20profile"), "url: {url}");
}

#[test]
fn build_authorization_url_trims_scope_whitespace() {
    let url = build_url(
        "https://example.com/auth",
        "c",
        "https://app/cb",
        "openid, email",
        "code",
    );
    assert!(url.contains("scope=openid%20email"), "url: {url}");
}

// -----------------------------------------------------------------------
// Optional params
// -----------------------------------------------------------------------

#[test]
fn build_authorization_url_omits_state_when_none() {
    let url = build_url(
        "https://example.com/auth",
        "c",
        "https://app/cb",
        "openid",
        "code",
    );
    assert!(!url.contains("state="), "url: {url}");
}

#[test]
fn build_authorization_url_includes_state_when_some() {
    let url = build_authorization_url(
        "https://example.com/auth".into(),
        "c".into(),
        "https://app/cb".into(),
        "openid".into(),
        "code".into(),
        Some("xyz123".into()),
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    assert!(url.contains("state=xyz123"), "url: {url}");
}

#[test]
fn build_authorization_url_maps_s256_code_challenge_method() {
    let url = build_authorization_url(
        "https://example.com/auth".into(),
        "c".into(),
        "https://app/cb".into(),
        "openid".into(),
        "code".into(),
        None,
        None,
        None,
        Some("challenge_value".into()),
        Some(CodeChallengeMethod::S256),
        None,
        None,
    )
    .unwrap();
    assert!(url.contains("code_challenge_method=S256"), "url: {url}");
}

#[test]
fn build_authorization_url_maps_plain_code_challenge_method() {
    let url = build_authorization_url(
        "https://example.com/auth".into(),
        "c".into(),
        "https://app/cb".into(),
        "openid".into(),
        "code".into(),
        None,
        None,
        None,
        Some("challenge_value".into()),
        Some(CodeChallengeMethod::Plain),
        None,
        None,
    )
    .unwrap();
    assert!(url.contains("code_challenge_method=plain"), "url: {url}");
}
