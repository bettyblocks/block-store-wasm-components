mod bindings {
    wasmtime_testing_helper::bindgen!("main");

    wasmtime_testing_helper::setup!(Main);
}

#[test]
fn concats_strings() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let interface = component.component.betty_blocks_concat_text_concat_text();
    let result = interface
        .call_concat_text(&mut component.store, "hi", "hooo")
        .unwrap();

    assert_eq!("hihooo", result);
}

#[test]
fn concats_strings_with_separator() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let interface = component.component.betty_blocks_concat_text_concat_text();
    let result = interface
        .call_concat_text_with_separator(&mut component.store, "ha", "hi", "   ")
        .unwrap();

    assert_eq!("ha   hi", result);
}

#[test]
fn concats_list_of_strings() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let interface = component.component.betty_blocks_concat_text_concat_text();
    let result = interface
        .call_concat_text_list(
            &mut component.store,
            &[
                String::from("hi"),
                String::from("ha"),
                String::from("hi"),
                String::from("123"),
            ],
        )
        .unwrap();

    assert_eq!("hihahi123", result);
}

#[test]
fn concats_list_of_strings_with_separator() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let interface = component.component.betty_blocks_concat_text_concat_text();
    let result = interface
        .call_concat_text_list_with_separator(
            &mut component.store,
            &[
                String::from("something"),
                String::from("what"),
                String::from("hello"),
                String::from("22!"),
            ],
            "-",
        )
        .unwrap();

    assert_eq!("something-what-hello-22!", result);
}
