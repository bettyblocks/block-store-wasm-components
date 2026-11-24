mod bindings {
    wasmtime_testing_helper::bindgen!("main");

    wasmtime_testing_helper::setup!(Main);
}

#[test]
fn it_evaluates_an_expression_with_variables() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let result = component
        .component
        .betty_blocks_cel_cel()
        .call_evaluate(
            &mut component.store,
            "price * quantity > 100",
            &String::from(r#"{"price": 30, "quantity": 4}"#),
        )
        .unwrap()
        .unwrap();

    assert_eq!(result, "true");
}

#[test]
fn it_returns_an_error_for_an_invalid_expression() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let error = component
        .component
        .betty_blocks_cel_cel()
        .call_evaluate(&mut component.store, "1 +", &String::from("{}"))
        .unwrap()
        .unwrap_err();

    assert!(error.starts_with("Expression: "), "{error}");
}
