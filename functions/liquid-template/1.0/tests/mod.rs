mod bindings {
    wasmtime::component::bindgen!({ path: "wit", world: "main" });
}

wasmtime_testing_helper::setup!(bindings);

#[test]
fn string_operations_templating_works() {
    let harness = harness();
    let mut component = instantiate(harness);

    let interface = component.component.betty_blocks_liquid_template_liquid_template();
    let rendered_template = interface
        .call_liquid_template(
            &mut component.store,
            Some("hi {{ something | upcase }}"),
            None,
            "{ \"something\": \"value\" }",
        )
        .unwrap()
        .unwrap();

    assert_eq!(rendered_template, "hi VALUE");
}

#[test]
fn it_gives_template_variable_precedence_over_template() {
    let harness = harness();
    let mut component = instantiate(harness);

    let interface = component.component.betty_blocks_liquid_template_liquid_template();
    let rendered_template = interface
        .call_liquid_template(
            &mut component.store,
            Some("hi"),
            Some("hi {{something}}"),
            "{ \"something\": \"value\" }",
        )
        .unwrap()
        .unwrap();

    assert_eq!(rendered_template, "hi value");
}

#[test]
fn it_does_not_work_with_no_json_object() {
    let harness = harness();
    let mut component = instantiate(harness);

    let interface = component.component.betty_blocks_liquid_template_liquid_template();
    let result = interface
        .call_liquid_template(
            &mut component.store,
            Some("This parameter does not matter"),
            None,
            "",
        )
        .unwrap();

    assert!(result.is_err());
}
