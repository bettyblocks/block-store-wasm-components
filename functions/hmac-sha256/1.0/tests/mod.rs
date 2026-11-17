mod bindings {
    wasmtime_testing_helper::bindgen!("main");

    wasmtime_testing_helper::setup!(Main);
}

#[test]
fn calculates_hmac_sha256() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let interface = component.component.betty_blocks_hmac_sha256_hmac_sha256();
    let result = interface
        .call_hmac_sha256(&mut component.store, "Jefe", "what do ya want for nothing?")
        .unwrap();

    assert_eq!(
        "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843",
        result
    );
}
