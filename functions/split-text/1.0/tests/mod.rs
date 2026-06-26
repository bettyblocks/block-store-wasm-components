mod bindings {
    wasmtime_testing_helper::bindgen!("main");

    wasmtime_testing_helper::setup!(Main);
}

#[test]
fn splits_all_text() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let interface = component.component.betty_blocks_split_text_split_text();
    let result = interface
        .call_split_all(&mut component.store, "hi hi hi hi", " ")
        .unwrap();

    assert_eq!(result, vec!["hi", "hi", "hi", "hi"]);
}
