use crate::exports::betty_blocks::generate_uuid::generate_uuid::{Guest, Output};

wit_bindgen::generate!({ generate_all });

struct Component;

impl Guest for Component {
    fn generate_uuid(name: String) -> Result<Output, String> {
        if name == "oops" {
            Err("Ooops. Something went wrong.".to_string())
        } else {
            Ok(Output {
                greet: format!("Hello, {}", name),
            })
        }
    }
}

export! {Component}
    