use crate::exports::betty_blocks::liquid::liquid::Guest;

wit_bindgen::generate!({ generate_all });

struct Liquid;

impl Guest for Liquid {
    // The template and template variable are both defined in the front-end, meaning we have to use
    // either or none as they're not required.
    fn liquid(template: Option<String>, template_variable: Option<String>, variables: String) -> Result<String, String> {
        if let Some(template) = template_variable {
            build_template(template, variables)
        } else if let Some(template) = template {
            build_template(template, variables)
        } else {
            Ok(String::default())
        }
    }
}

fn build_template(template: String, variables: String) -> Result<String, String> {
    // This does not necessarily validate the JSON contents.
    let variables_json: serde_json::Map<String, serde_json::Value> =
        // This just gives line and character number, `JSON: ` makes it more obvious what the
        // error is to the user.
        serde_json::from_str(&variables).map_err(|error| format!("JSON: {}", error.to_string()))?;

    let globals =
        liquid::model::to_object(&variables_json).map_err(|error| error.to_string())?;

    Ok(render_template(&template, &globals).map_err(|error| error.to_string())?)
}

fn render_template(
    template: &str,
    globals: &liquid::model::Object,
) -> Result<String, liquid::Error> {
    liquid::ParserBuilder::with_stdlib()
        .build()?
        .parse(&template)?
        .render(&globals)
}

export! {Liquid}

#[test]
fn can_render_template_without_variables() {
    let result = Liquid::liquid(Some(String::from("hi")), None, String::from("{}")).unwrap();
    assert_eq!(result, "hi");
}

#[test]
fn cannot_render_template_with_missing_variable() {
    let result = Liquid::liquid(Some(String::from("hi {{something}}")), None, String::from("{}"));
    assert!(result.is_err());
}

#[test]
fn cannot_render_template_with_invalid_json() {
    let result = Liquid::liquid(
        Some(String::from("hi {{something}}")),
        None,
        String::from("{ \"incorrect_value\" }"),
    );
    assert_eq!(
        result.unwrap_err(),
        "JSON: expected `:` at line 1 column 21"
    );
}

#[test]
fn can_render_template_with_variable() {
    let result = Liquid::liquid(
        Some(String::from("hi {{something}}")),
        None,
        String::from("{ \"something\": \"value\" }"),
    )
    .unwrap();
    assert_eq!(result, "hi value");
}

#[test]
fn can_render_template_with_extra_variables() {
    let result = Liquid::liquid(
        Some(String::from("hi {{something}}")),
        None,
        String::from("{ \"something\": \"value\", \"something_else\": \"value\" }"),
    )
    .unwrap();
    assert_eq!(result, "hi value");
}

#[test]
fn template_variable_definition_takes_precedence() {
    let result = Liquid::liquid(
        Some(String::from("hi")),
        Some(String::from("hi {{something}}")),
        String::from("{ \"something\": \"value\" }"),
    )
    .unwrap();
    assert_eq!(result, "hi value");
}

#[test]
fn no_template_or_template_variable_defintion_returns_empty_string() {
    let result = Liquid::liquid(
        None,
        None,
        String::from("{}"),
    )
    .unwrap();
    assert_eq!(result, "");
}
