mod bindings {
    wasmtime_testing_helper::bindgen!("main");

    wasmtime_testing_helper::setup!(Main);
}

use crate::bindings::betty_blocks_types::types::types::KeyValue;
use serde_json::json;

fn format_endpoint_result(
    status_code: u16,
    body: serde_json::Value,
    headers: &[(&str, Option<serde_json::Value>)],
) -> Result<String, String> {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let headers: Vec<KeyValue> = headers
        .iter()
        .map(|(key, value)| KeyValue {
            key: String::from(*key),
            value: value.as_ref().map(|value| value.to_string()),
        })
        .collect();

    component
        .component
        .betty_blocks_format_endpoint_result_format_endpoint_result()
        .call_format_endpoint_result(
            &mut component.store,
            status_code,
            &body.to_string(),
            &headers,
        )
        .unwrap()
}

#[test]
fn formats_an_object_body_and_headers() {
    let result = format_endpoint_result(
        201,
        json!({ "result": true }),
        &[("Content-Type", Some(json!("application/json")))],
    )
    .unwrap();

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&result).unwrap(),
        json!({
            "statusCode": 201,
            "body": "{\"result\":true}",
            "headers": [["Content-Type", "application/json"]]
        })
    );
}

#[test]
fn formats_a_text_body_without_double_encoding() {
    let result = format_endpoint_result(200, json!("Ok"), &[]).unwrap();

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&result).unwrap(),
        json!({ "statusCode": 200, "body": "Ok", "headers": [] })
    );
}

#[test]
fn formats_non_text_header_values_and_empty_header_values() {
    let result = format_endpoint_result(
        200,
        json!(null),
        &[("X-Count", Some(json!(5))), ("X-Empty", None)],
    )
    .unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["body"], json!("null"));
    assert_eq!(
        parsed["headers"],
        json!([["X-Count", 5], ["X-Empty", null]])
    );
}
