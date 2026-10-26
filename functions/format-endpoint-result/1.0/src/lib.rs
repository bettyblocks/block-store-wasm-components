mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::FormatEndpointResult;
    export!(FormatEndpointResult);
}

use crate::bindings::betty_blocks_types::types::types::{
    BettyMap, BettyValue, JsonString, KeyValue,
};
use crate::bindings::exports::betty_blocks::format_endpoint_result::format_endpoint_result::Guest;

#[derive(Debug, Clone, serde::Serialize)]
#[cfg_attr(test, derive(serde::Deserialize))]
struct FormattedEndpointResult {
    #[serde(rename = "statusCode")]
    status_code: u16,
    /// The body is explicitly left unparsed (and so will be serialized as a string within a JSON string) to be compatible with the old JavaScript action.
    body: String,
    headers: Vec<(String, serde_json::Value)>,
}

fn format_body(body: BettyValue) -> String {
    match serde_json::from_str::<serde_json::Value>(&body) {
        Ok(serde_json::Value::String(text)) => text,
        _ => body,
    }
}

fn format_header(header: KeyValue) -> Result<(String, serde_json::Value), String> {
    let value = match header.value {
        Some(value) => serde_json::from_str(&value)
            .map_err(|_| format!("The value of header `{}` is not valid JSON", header.key))?,
        None => serde_json::Value::Null,
    };

    Ok((header.key, value))
}

struct FormatEndpointResult;

impl Guest for FormatEndpointResult {
    fn format_endpoint_result(
        status_code: u16,
        body: BettyValue,
        headers: BettyMap,
    ) -> Result<JsonString, String> {
        let headers = headers
            .into_iter()
            .map(format_header)
            .collect::<Result<Vec<_>, String>>()?;

        serde_json::to_string(&FormattedEndpointResult {
            status_code,
            body: format_body(body),
            headers,
        })
        .map_err(|_| String::from("Could not serialize formatted endpoint result to JSON"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn header(key: &str, value: Option<&str>) -> KeyValue {
        KeyValue {
            key: String::from(key),
            value: value.map(String::from),
        }
    }

    fn format(status_code: u16, body: &str, headers: BettyMap) -> FormattedEndpointResult {
        serde_json::from_str(
            &FormatEndpointResult::format_endpoint_result(status_code, body.into(), headers)
                .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn object_body_is_kept_as_json_text_test() {
        let result = format(201, r#"{"result":true}"#, vec![]);

        assert_eq!(result.status_code, 201);
        assert_eq!(result.body, r#"{"result":true}"#);
        assert!(result.headers.is_empty());
    }

    #[test]
    fn text_body_is_unwrapped_test() {
        assert_eq!(format(200, r#""Ok""#, vec![]).body, "Ok");
    }

    #[test]
    fn text_body_containing_json_is_returned_as_is_test() {
        assert_eq!(
            format(200, r#""{\"result\":true}""#, vec![]).body,
            r#"{"result":true}"#
        );
    }

    #[test]
    fn non_text_scalar_body_is_kept_as_json_text_test() {
        assert_eq!(format(200, "42", vec![]).body, "42");
        assert_eq!(format(200, "true", vec![]).body, "true");
        assert_eq!(format(200, "null", vec![]).body, "null");
    }

    #[test]
    fn body_that_is_not_json_is_returned_as_is_test() {
        assert_eq!(format(200, "plain text", vec![]).body, "plain text");
    }

    #[test]
    fn header_values_are_parsed_from_json_test() {
        let result = format(
            200,
            "null",
            vec![
                header("Content-Type", Some(r#""application/json""#)),
                header("X-Count", Some("5")),
                header("X-Empty", None),
            ],
        );

        assert_eq!(
            result.headers,
            vec![
                (String::from("Content-Type"), json!("application/json")),
                (String::from("X-Count"), json!(5)),
                (String::from("X-Empty"), serde_json::Value::Null),
            ]
        );
    }

    #[test]
    fn output_matches_the_old_javascript_action_test() {
        let output = FormatEndpointResult::format_endpoint_result(
            200,
            r#""Ok""#.into(),
            vec![header("Content-Type", Some(r#""application/json""#))],
        )
        .unwrap();

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&output).unwrap(),
            json!({
                "statusCode": 200,
                "body": "Ok",
                "headers": [["Content-Type", "application/json"]]
            })
        );
    }

    #[test]
    fn invalid_header_value_returns_an_error_test() {
        assert_eq!(
            FormatEndpointResult::format_endpoint_result(
                200,
                "null".into(),
                vec![header("Content-Type", Some("application/json"))],
            ),
            Err(String::from(
                "The value of header `Content-Type` is not valid JSON"
            ))
        );
    }
}
