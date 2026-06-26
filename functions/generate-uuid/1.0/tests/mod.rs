mod bindings {
    wasmtime_testing_helper::bindgen!("main");

    wasmtime_testing_helper::setup!(Main);
}

#[test]
fn is_valid_uuidv4() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let interface = component
        .component
        .betty_blocks_generate_uuid_generate_uuid();
    let uuid = interface.call_generate_uuid(&mut component.store).unwrap();

    assert_eq!(uuid.len(), 32 + 4);

    for (index, character) in uuid.chars().enumerate() {
        if index == 8 || index == 8 + 5 || index == 8 + 5 + 5 || index == 8 + 5 + 5 + 5 {
            assert_eq!(character, '-');
        } else if index == 14 {
            assert_eq!(character, '4');
        } else {
            assert!(character.is_ascii_hexdigit());
        }
    }
}
