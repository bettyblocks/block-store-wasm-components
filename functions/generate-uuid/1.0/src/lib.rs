use crate::exports::betty_blocks::generate_uuid::generate_uuid::{Guest, Output};

wit_bindgen::generate!({ generate_all });

struct Component;

impl Guest for Component {
    fn generate_uuid() -> Output {
        Output {
            uuid: String::from(uuid::Uuid::new_v4())
        }
    }
}

export! {Component}
    
