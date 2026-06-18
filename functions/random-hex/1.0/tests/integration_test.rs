mod bindings {
    wasmtime::component::bindgen!({ path: "wit", world: "main" });
}

wasmtime_testing_helper::setup!(bindings);

#[test]
fn generate_random_hex_returns_correct_length() {
    let mut component = instantiate(harness());

    let interface = component.component.betty_blocks_random_hex_random_hex();
    let result = interface
        .call_generate_random_hex(&mut component.store, 16)
        .expect("failed to call generate-random-hex");

    assert_eq!(result.len(), 16);
}

#[test]
fn generate_random_hex_produces_valid_hex() {
    let mut component = instantiate(harness());

    let interface = component.component.betty_blocks_random_hex_random_hex();
    let result = interface
        .call_generate_random_hex(&mut component.store, 32)
        .expect("failed to call generate-random-hex");

    assert!(
        u128::from_str_radix(&result, 16).is_ok(),
        "result '{result}' is not valid hex"
    );
}

#[test]
fn generate_random_hex_with_zero_size_returns_empty() {
    let mut component = instantiate(harness());

    let interface = component.component.betty_blocks_random_hex_random_hex();
    let result = interface
        .call_generate_random_hex(&mut component.store, 0)
        .expect("failed to call generate-random-hex");

    assert!(result.is_empty());
}
