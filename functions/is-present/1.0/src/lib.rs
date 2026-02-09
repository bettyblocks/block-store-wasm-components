use crate::exports::betty_blocks::is_present::is_present::{Guest, JsonString};
use serde_json::Value;

wit_bindgen::generate!({ generate_all });

struct Component;

impl Guest for Component {
    fn is_present(value: JsonString) -> Result<bool, String> {
        let value: Value = serde_json::from_str(&value)
            .map_err(|_| String::from("The value was not valid json"))?;
        Ok(match value {
            Value::Null => false,
            Value::Array(arr) => !arr.is_empty(),
            _ => true,
        })
    }
}

export! {Component}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_is_not_present_test() {
        assert!(!Component::is_present(String::from("null")).unwrap());
    }

    #[test]
    fn empty_array_is_not_present_test() {
        assert!(!Component::is_present(String::from("[]")).unwrap());
    }

    #[test]
    fn array_with_items_is_present_test() {
        assert!(Component::is_present(String::from("[null]")).unwrap());
    }

    #[test]
    fn number_is_present_test() {
        assert!(Component::is_present(String::from("1")).unwrap());
    }

    #[test]
    fn bool_is_present_test() {
        assert!(Component::is_present(String::from("true")).unwrap());
    }

    #[test]
    fn object_is_present_test() {
        assert!(Component::is_present(String::from("{}")).unwrap());
    }

    #[test]
    fn string_is_present_test() {
        assert!(Component::is_present(String::from("\"\"")).unwrap());
    }
}
