use crate::exports::betty_blocks::liquid::liquid::{Guest, Output};

wit_bindgen::generate!({ generate_all });

struct Liquid;

impl Guest for Liquid {
    fn liquid() -> Output {
        Output {
            placeholder: String::default()
        }
    }
}

export! {Liquid}

