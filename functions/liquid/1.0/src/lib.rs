// TODO: Change definition here to match Crate name?
use crate::exports::betty_blocks::liquid::liquid::{Guest};

wit_bindgen::generate!({ generate_all });

struct Liquid;

impl Guest for Liquid {
    fn liquid(template: String, variables: String) -> Result<String, String> {
        // Parse the string as a JSON object to be used by the rendering.
        let globals = liquid::to_object(&variables).map_err(|error| error.to_string())?;

        let rendered_template = liquid::ParserBuilder::with_stdlib()
            .build().map_err(|error| error.to_string())?
            .parse(&template).map_err(|error| error.to_string())?
            .render(&globals).map_err(|error| error.to_string())?;

        Ok(rendered_template)
    }
}

export! {Liquid}
