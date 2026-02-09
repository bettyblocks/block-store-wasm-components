use crate::exports::betty_blocks::is_present::is_present::{Guest, JsonString, Output};
use serde_json::Value;

wit_bindgen::generate!({ generate_all });

struct Component;

impl Guest for Component {
    fn is_present(value: JsonString) -> Result<Output, String> {
        let value: Value = serde_json::from_str(&value)
            .map_err(|_| String::from("The value was not valid json"))?;
        Ok(Output {
            result: match value {
                Value::Null => false,
                Value::Array(arr) => !arr.is_empty(),
                _ => true,
            },
        })
    }
}

export! {Component}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_is_not_present_test() {
        assert!(!Component::is_present(String::from("null")).unwrap().result);
    }

    #[test]
    fn empty_array_is_not_present_test() {
        assert!(!Component::is_present(String::from("[]")).unwrap().result);
    }

    #[test]
    fn array_with_items_is_present_test() {
        assert!(
            Component::is_present(String::from("[null]"))
                .unwrap()
                .result
        );
    }

    #[test]
    fn number_is_present_test() {
        assert!(Component::is_present(String::from("1")).unwrap().result);
    }

    #[test]
    fn bool_is_present_test() {
        assert!(Component::is_present(String::from("true")).unwrap().result);
    }

    #[test]
    fn object_is_present_test() {
        assert!(Component::is_present(String::from("{}")).unwrap().result);
    }

    #[test]
    fn string_is_present_test() {
        assert!(Component::is_present(String::from("\"\"")).unwrap().result);
    }
}
