mod bindings {
    wasmtime::component::bindgen!({ path: "wit", world: "main" });
}

wasmtime_testing_helper::setup!(bindings);

#[test]
fn gives_the_correct_length() {
    let harness = harness();
    let mut component = instantiate(harness);

    let interface = component
        .component
        .betty_blocks_generate_random_hex_generate_random_hex();
    let random_hex = interface
        .call_generate_random_hex(&mut component.store, 20)
        .unwrap();

    // Parse as hex to verify it's greater than 0.
    let parsed = u128::from_str_radix(&random_hex, 16).unwrap();
    assert!(parsed > 0);

    assert_eq!(random_hex.len(), 20);
}
