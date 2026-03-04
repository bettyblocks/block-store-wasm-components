use crate::params::build_query_string;

#[test]
fn build_query_string_empty_returns_empty_string() {
    assert_eq!(build_query_string(&[]), "");
}

#[test]
fn build_query_string_single_param() {
    assert_eq!(build_query_string(&[("key", "value")]), "key=value");
}

#[test]
fn build_query_string_multiple_params_joined_with_ampersand() {
    assert_eq!(build_query_string(&[("a", "1"), ("b", "2")]), "a=1&b=2");
}

#[test]
fn build_query_string_encodes_value() {
    assert_eq!(
        build_query_string(&[("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer")]),
        "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Ajwt-bearer"
    );
}

#[test]
fn build_query_string_encodes_key() {
    assert_eq!(
        build_query_string(&[("hello world", "x")]),
        "hello%20world=x"
    );
}

#[test]
fn build_query_string_jwt_bearer_params_have_exactly_one_ampersand() {
    let body = build_query_string(&[
        ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
        ("assertion", "header.payload.signature"),
    ]);
    assert_eq!(body.matches('&').count(), 1);
}

#[test]
fn build_query_string_grant_type_comes_before_assertion() {
    let body = build_query_string(&[
        ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
        ("assertion", "header.payload.signature"),
    ]);
    let grant_pos = body.find("grant_type=").unwrap();
    let assertion_pos = body.find("assertion=").unwrap();
    assert!(grant_pos < assertion_pos);
}
