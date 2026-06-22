mod bindings {
    wasmtime::component::bindgen!({ path: "wit", world: "main" });
}

wasmtime_testing_helper::setup!(bindings);

const RANDOM_HEX_INTERFACE: &str = "betty-blocks:random-hex/random-hex@1.0.0";

#[test]
fn exported_uuid_with_random_hex_is_valid() {
    let mut harness = harness();
    harness.stub::<(u32,), (String,)>(
        RANDOM_HEX_INTERFACE,
        "generate-random-hex",
        ("DEADBEEF".to_string(),),
    );
    let mut component = instantiate(harness);

    let interface = component.component.betty_blocks_generate_uuid_generate_uuid();
    let result = interface
        .call_generate_uuid(&mut component.store)
        .expect("failed to call generate-uuid");

    let parts: Vec<&str> = result.rsplitn(2, '-').collect();
    let random_hex_part = parts[0];
    let uuid_part = parts[1];

    assert_eq!(random_hex_part, "DEADBEEF");
    assert_eq!(uuid_part.len(), 36);
    assert_eq!(component.call_count(RANDOM_HEX_INTERFACE, "generate-random-hex"), 1);
}

#[test]
fn uuid_batch_tracks_state_across_calls() {
    let mut harness = harness();
    harness.mock(
        RANDOM_HEX_INTERFACE,
        "generate-random-hex",
        |_context, (size,): (u32,)| Ok(("A".repeat(size as usize),)),
    );
    let mut component = instantiate(harness);

    let interface = component.component.betty_blocks_generate_uuid_generate_uuid();
    let batch_api = interface.uuid_batch();

    let batch = batch_api
        .call_constructor(&mut component.store)
        .expect("constructor failed");

    let count = batch_api
        .call_count(&mut component.store, batch)
        .expect("count failed");
    assert_eq!(count, 0);

    let first = batch_api
        .call_generate_next(&mut component.store, batch)
        .expect("generate-next failed");
    let second = batch_api
        .call_generate_next(&mut component.store, batch)
        .expect("generate-next failed");

    assert_ne!(first, second);

    let count = batch_api
        .call_count(&mut component.store, batch)
        .expect("count failed");
    assert_eq!(count, 2);

    let collected = batch_api
        .call_collect(&mut component.store, batch)
        .expect("collect failed");
    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0], first);
    assert_eq!(collected[1], second);
    assert_eq!(component.call_count(RANDOM_HEX_INTERFACE, "generate-random-hex"), 2);
}

#[test]
fn multiple_uuid_batches_have_independent_state() {
    let mut harness = harness();
    harness.mock(
        RANDOM_HEX_INTERFACE,
        "generate-random-hex",
        |_context, (size,): (u32,)| Ok(("A".repeat(size as usize),)),
    );
    let mut component = instantiate(harness);

    let interface = component.component.betty_blocks_generate_uuid_generate_uuid();
    let batch_api = interface.uuid_batch();

    let batch_a = batch_api.call_constructor(&mut component.store).unwrap();
    let batch_b = batch_api.call_constructor(&mut component.store).unwrap();

    batch_api.call_generate_next(&mut component.store, batch_a).unwrap();
    batch_api.call_generate_next(&mut component.store, batch_a).unwrap();
    batch_api.call_generate_next(&mut component.store, batch_a).unwrap();
    batch_api.call_generate_next(&mut component.store, batch_b).unwrap();

    let count_a = batch_api.call_count(&mut component.store, batch_a).unwrap();
    let count_b = batch_api.call_count(&mut component.store, batch_b).unwrap();

    assert_eq!(count_a, 3);
    assert_eq!(count_b, 1);
    assert_eq!(component.call_count(RANDOM_HEX_INTERFACE, "generate-random-hex"), 4);
}
