mod bindings {
    wasmtime_testing_helper::bindgen!("main");

    wasmtime_testing_helper::setup!(Main);
}

#[test]
fn gives_the_correct_length() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

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
