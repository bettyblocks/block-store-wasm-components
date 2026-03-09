use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

/// RFC 3986 unreserved characters: A-Za-z0-9 and `-`, `_`, `.`, `~`.
const UNRESERVED: &percent_encoding::AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');

pub fn build_query_string(params: &[(&str, &str)]) -> String {
    params
        .iter()
        .map(|(k, v)| {
            format!(
                "{}={}",
                utf8_percent_encode(k, UNRESERVED),
                utf8_percent_encode(v, UNRESERVED)
            )
        })
        .collect::<Vec<_>>()
        .join("&")
}

pub fn create_base_params(
    client_id: &str,
    redirect_uri: &str,
    scope: &str,
    response_type: &str,
) -> String {
    format!(
        "client_id={}&redirect_uri={}&scope={}&response_type={}",
        utf8_percent_encode(client_id, UNRESERVED),
        utf8_percent_encode(redirect_uri, UNRESERVED),
        utf8_percent_encode(scope, UNRESERVED),
        utf8_percent_encode(response_type, UNRESERVED)
    )
}

pub fn push_param(params: &mut String, param_name: &str, param_value: &str) {
    params.push('&');
    params.extend(utf8_percent_encode(param_name, UNRESERVED));
    params.push('=');
    params.extend(utf8_percent_encode(param_value, UNRESERVED));
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
