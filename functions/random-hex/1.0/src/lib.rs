use crate::exports::betty_blocks::random_hex::random_hex::{Guest, Output};

wit_bindgen::generate!({ generate_all });

struct Component;

impl Guest for Component {
    fn random_hex(name: String) -> Result<Output, String> {
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
    