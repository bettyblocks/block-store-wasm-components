mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::LiquidTemplate;
    export!(LiquidTemplate);
}

use crate::bindings::exports::betty_blocks::liquid_template::liquid_template::Guest;

struct LiquidTemplate;

impl Guest for LiquidTemplate {
    // The template and template variable are both defined in the front-end, meaning we have to use
    // either or none as they're not required.
    fn liquid_template(
        template: Option<String>,
        template_variable: Option<String>,
        variables: String,
    ) -> Result<String, String> {
        let template_variable = template_variable
            .map(|value| parse_template_variable(&value))
            .transpose()?
            .flatten();

        if let Some(template) = template_variable.or(template) {
            build_template(template, variables)
        } else {
            Ok(String::default())
        }
    }
}

fn parse_template_variable(value: &str) -> Result<Option<String>, String> {
    match serde_json::from_str(value)
        .map_err(|error| format!("Template variable JSON: {}", error))?
    {
        serde_json::Value::String(template) => Ok(Some(template)),
        serde_json::Value::Null => Ok(None),
        _ => Err(String::from("Template variable must be text")),
    }
}

fn build_template(template: String, variables: String) -> Result<String, String> {
    // This does not necessarily validate the JSON contents.
    let variables_json: serde_json::Map<String, serde_json::Value> =
        // This just gives line and character number, `JSON: ` makes it more obvious what the
        // error is to the user.
        serde_json::from_str(&variables).map_err(|error| format!("JSON: {}", error))?;

    let globals = liquid::model::to_object(&variables_json).map_err(|error| error.to_string())?;

    render_template(&template, &globals).map_err(|error| error.to_string())
}

fn render_template(
    template: &str,
    globals: &liquid::model::Object,
) -> Result<String, liquid::Error> {
    liquid::ParserBuilder::with_stdlib()
        .build()?
        .parse(template)?
        .render(&globals)
}

#[test]
fn can_render_template_without_variables() {
    let result =
        LiquidTemplate::liquid_template(Some(String::from("hi")), None, String::from("{}"))
            .unwrap();
    assert_eq!(result, "hi");
}

#[test]
fn cannot_render_template_with_missing_variable() {
    let result = LiquidTemplate::liquid_template(
        Some(String::from("hi {{something}}")),
        None,
        String::from("{}"),
    );
    assert!(result.is_err());
}

#[test]
fn cannot_render_template_with_invalid_json() {
    let result = LiquidTemplate::liquid_template(
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
    let result = LiquidTemplate::liquid_template(
        Some(String::from("hi {{something}}")),
        None,
        String::from("{ \"something\": \"value\" }"),
    )
    .unwrap();
    assert_eq!(result, "hi value");
}

#[test]
fn can_render_template_with_extra_variables() {
    let result = LiquidTemplate::liquid_template(
        Some(String::from("hi {{something}}")),
        None,
        String::from("{ \"something\": \"value\", \"something_else\": \"value\" }"),
    )
    .unwrap();
    assert_eq!(result, "hi value");
}

#[test]
fn template_variable_definition_takes_precedence() {
    let result = LiquidTemplate::liquid_template(
        Some(String::from("hi")),
        Some(String::from("\"hi {{something}}\"")),
        String::from("{ \"something\": \"value\" }"),
    )
    .unwrap();
    assert_eq!(result, "hi value");
}

#[test]
fn no_template_or_template_variable_defintion_returns_empty_string() {
    let result = LiquidTemplate::liquid_template(None, None, String::from("{}")).unwrap();
    assert_eq!(result, "");
}

#[test]
fn template_variable_null_falls_back_to_template() {
    let result = LiquidTemplate::liquid_template(
        Some(String::from("hi {{something}}")),
        Some(String::from("null")),
        String::from("{ \"something\": \"value\" }"),
    )
    .unwrap();
    assert_eq!(result, "hi value");
}

#[test]
fn template_variable_without_template_is_used() {
    let result = LiquidTemplate::liquid_template(
        None,
        Some(String::from("\"Hello {{ name }}\"")),
        String::from("{ \"name\": \"Betty\" }"),
    )
    .unwrap();
    assert_eq!(result, "Hello Betty");
}

#[test]
fn template_variable_that_is_not_text_is_an_error() {
    let result = LiquidTemplate::liquid_template(
        Some(String::from("hi")),
        Some(String::from("42")),
        String::from("{}"),
    );
    assert_eq!(result.unwrap_err(), "Template variable must be text");
}

#[test]
fn template_variable_that_is_not_json_is_an_error() {
    let result = LiquidTemplate::liquid_template(
        Some(String::from("hi")),
        Some(String::from("hi {{something}}")),
        String::from("{}"),
    );
    assert!(result.unwrap_err().starts_with("Template variable JSON: "));
}
