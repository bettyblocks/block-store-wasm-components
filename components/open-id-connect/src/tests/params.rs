use crate::params::{build_query_string, percent_encode};

// -----------------------------------------------------------------------
// percent_encode
// -----------------------------------------------------------------------

#[test]
fn percent_encode_passes_through_unreserved_chars() {
    assert_eq!(percent_encode("abcXYZ0-_.~"), "abcXYZ0-_.~");
}

#[test]
fn percent_encode_encodes_space() {
    assert_eq!(percent_encode(" "), "%20");
}

#[test]
fn percent_encode_encodes_colon() {
    assert_eq!(percent_encode(":"), "%3A");
}

#[test]
fn percent_encode_encodes_equals() {
    assert_eq!(percent_encode("="), "%3D");
}

#[test]
fn percent_encode_encodes_ampersand() {
    assert_eq!(percent_encode("&"), "%26");
}

#[test]
fn percent_encode_encodes_plus() {
    assert_eq!(percent_encode("+"), "%2B");
}

#[test]
fn percent_encode_encodes_urn_grant_type() {
    // Colons in the JWT-bearer grant type URN must be percent-encoded
    assert_eq!(
        percent_encode("urn:ietf:params:oauth:grant-type:jwt-bearer"),
        "urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Ajwt-bearer"
    );
}

// -----------------------------------------------------------------------
// build_query_string
// -----------------------------------------------------------------------

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
