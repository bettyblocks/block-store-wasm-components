use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

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
