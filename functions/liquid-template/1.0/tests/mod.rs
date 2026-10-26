mod bindings {
    wasmtime_testing_helper::bindgen!("main");

    wasmtime_testing_helper::setup!(Main);
}

#[test]
fn string_operations_templating_works() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let interface = component
        .component
        .betty_blocks_liquid_template_liquid_template();
    let rendered_template = interface
        .call_liquid_template(
            &mut component.store,
            Some("hi {{ something | upcase }}"),
            None,
            &String::from("{ \"something\": \"value\" }"),
        )
        .unwrap()
        .unwrap();

    assert_eq!(rendered_template, "hi VALUE");
}

#[test]
fn it_gives_template_variable_precedence_over_template() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let interface = component
        .component
        .betty_blocks_liquid_template_liquid_template();
    let rendered_template = interface
        .call_liquid_template(
            &mut component.store,
            Some("hi"),
            Some(&String::from("\"hi {{something}}\"")),
            &String::from("{ \"something\": \"value\" }"),
        )
        .unwrap()
        .unwrap();

    assert_eq!(rendered_template, "hi value");
}

#[test]
fn it_does_not_work_with_no_json_object() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let interface = component
        .component
        .betty_blocks_liquid_template_liquid_template();
    let result = interface
        .call_liquid_template(
            &mut component.store,
            Some("This parameter does not matter"),
            None,
            &String::from(""),
        )
        .unwrap();

    assert!(result.is_err());
}

#[test]
fn it_renders_a_template_variable_that_arrives_as_json_text() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let interface = component
        .component
        .betty_blocks_liquid_template_liquid_template();
    let rendered_template = interface
        .call_liquid_template(
            &mut component.store,
            None,
            Some(&String::from("\"Hello {{ name }}\"")),
            &String::from("{\"name\":\"Betty\"}"),
        )
        .unwrap()
        .unwrap();

    assert_eq!(rendered_template, "Hello Betty");
}

#[test]
fn it_falls_back_to_the_template_when_the_template_variable_is_null() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let interface = component
        .component
        .betty_blocks_liquid_template_liquid_template();
    let rendered_template = interface
        .call_liquid_template(
            &mut component.store,
            Some("hi {{ something }}"),
            Some(&String::from("null")),
            &String::from("{\"something\":\"value\"}"),
        )
        .unwrap()
        .unwrap();

    assert_eq!(rendered_template, "hi value");
}

#[test]
fn it_rejects_a_template_variable_that_is_not_text() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let interface = component
        .component
        .betty_blocks_liquid_template_liquid_template();
    let result = interface
        .call_liquid_template(
            &mut component.store,
            Some("hi"),
            Some(&String::from("42")),
            &String::from("{}"),
        )
        .unwrap();

    assert_eq!(result.unwrap_err(), "Template variable must be text");
}
