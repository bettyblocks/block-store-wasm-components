mod bindings {
    wasmtime::component::bindgen!({ path: "wit", world: "main" });
}

use crate::bindings::exports::betty_blocks::format_endpoint_result::format_endpoint_result::Header;

wasmtime_testing_helper::setup!(bindings);

#[test]
fn it_works() {
    let harness = harness();
    let mut component = instantiate(harness);

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
