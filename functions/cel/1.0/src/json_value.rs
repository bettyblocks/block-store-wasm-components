use cel_interpreter::Value;
use serde_json::Value as Json;
use std::collections::HashMap;

/// Converts JSON to a CEL value. Integers become `Int` (not `UInt`) so they mix with CEL's
/// integer literals in arithmetic.
pub fn to_cel(json: Json) -> Value {
    match json {
        Json::Null => Value::Null,
        Json::Bool(boolean) => Value::Bool(boolean),
        Json::Number(number) => match (number.as_i64(), number.as_u64()) {
            (Some(int), _) => Value::Int(int),
            (None, Some(uint)) => Value::UInt(uint),
            _ => Value::Float(number.as_f64().unwrap_or(f64::NAN)),
        },
        Json::String(text) => Value::from(text),
        Json::Array(items) => Value::from(items.into_iter().map(to_cel).collect::<Vec<_>>()),
        Json::Object(entries) => Value::from(
            entries
                .into_iter()
                .map(|(key, value)| (key, to_cel(value)))
                .collect::<HashMap<_, _>>(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn small_integers_become_int() {
        assert_eq!(to_cel(json!(1)), Value::Int(1));
        assert_eq!(to_cel(json!(-1)), Value::Int(-1));
    }

    #[test]
    fn integers_above_i64_become_uint() {
        assert_eq!(to_cel(json!(u64::MAX)), Value::UInt(u64::MAX));
    }

    #[test]
    fn decimals_become_float() {
        assert_eq!(to_cel(json!(1.5)), Value::Float(1.5));
    }

    #[test]
    fn nested_integers_become_int() {
        assert_eq!(to_cel(json!([1])), Value::from(vec![Value::Int(1)]));
    }
}
