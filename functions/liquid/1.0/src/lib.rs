// TODO: Change definition here to match Crate name?
use crate::exports::betty_blocks::liquid::liquid::Guest;

wit_bindgen::generate!({ generate_all });

struct Liquid;

impl Guest for Liquid {
    fn liquid(template: String, variables: String) -> Result<String, String> {
        // This does not necessarily validate the JSON contents.
        let variables_json: serde_json::Value =
            serde_json::from_str(&variables).map_err(|error| error.to_string())?;

        let globals =
            liquid::model::to_object(&variables_json).map_err(|error| error.to_string())?;

        let rendered_template = liquid::ParserBuilder::with_stdlib()
            .build()
            .map_err(|error| error.to_string())?
            .parse(&template)
            .map_err(|error| error.to_string())?
            .render(&globals)
            .map_err(|error| error.to_string())?;

        Ok(rendered_template)
    }
}

export! {Liquid}

#[test]
fn can_render_template_without_variables() {
    let result = Liquid::liquid(String::from("hi"), String::from("{}")).unwrap();
    assert_eq!(result, "hi");
}

#[test]
fn cannot_render_template_with_missing_variable() {
    let result = Liquid::liquid(String::from("hi {{something}}"), String::from("{}"));
    assert!(result.is_err());
}

#[test]
fn can_render_template_with_variable() {
    let result = Liquid::liquid(
        String::from("hi {{something}}"),
        String::from("{ \"something\": \"value\" }"),
    )
    .unwrap();
    assert_eq!(result, "hi value");
}

#[test]
fn can_render_template_with_extra_variables() {
    let result = Liquid::liquid(
        String::from("hi {{something}}"),
        String::from("{ \"something\": \"value\", \"something_else\": \"value\" }"),
    )
    .unwrap();
    assert_eq!(result, "hi value");
}
