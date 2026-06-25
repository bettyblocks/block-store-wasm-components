mod bindings {
    wasmtime_testing_helper::bindgen!("main");

    wasmtime_testing_helper::setup!(Main);
}

use crate::bindings::exports::betty_blocks::format_endpoint_result::format_endpoint_result::Header;

#[test]
fn formats_endpoints_correctly() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let interface = component
        .component
        .betty_blocks_format_endpoint_result_format_endpoint_result();
    let result = interface
        .call_format_endpoint_result(
            &mut component.store,
            200,
            &String::from("{\"result\": true}"),
            &[Header {
                key: String::from("Accept"),
                value: String::from("\"application/json\""),
            }],
        )
        .unwrap()
        .unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed["headers"][0].is_array());
}
