mod bindings {
    wasmtime::component::bindgen!("main");

}

wasmtime_testing_helper::setup!(bindings);

#[test]
fn redirects_url_correctly() {
    let harness = harness();
    let mut component = instantiate(harness);

    let interface = component.component.betty_blocks_redirect_url_redirect_url();
    let result = interface
        .call_redirect_url(&mut component.store, "http://example.com")
        .unwrap()
        .unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["statusCode"], 302);
    assert_eq!(parsed["body"], "Redirect");
    assert_eq!(
        parsed["headers"],
        serde_json::json!([["Location", "http://example.com"]])
    );
}
