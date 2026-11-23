mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::Cel;
    export!(Cel);
}

use crate::bindings::exports::betty_blocks::cel::cel::Guest;
use cel_interpreter::{Context, Program};

mod json_value;

struct Cel;

impl Guest for Cel {
    fn evaluate(expression: String, variables: String) -> Result<String, String> {
        let context = build_context(&variables)?;
        let program =
            Program::compile(&expression).map_err(|error| format!("Expression: {error}"))?;
        let value = program
            .execute(&context)
            .map_err(|error| format!("Evaluation: {error}"))?;
        let json = value
            .json()
            .map_err(|error| format!("Result cannot be represented as JSON: {error}"))?;

        Ok(json.to_string())
    }
}

fn build_context(variables: &str) -> Result<Context<'static, 'static>, String> {
    let variables: serde_json::Map<String, serde_json::Value> =
        match serde_json::from_str(variables) {
            Ok(serde_json::Value::Object(map)) => map,
            Ok(_) => return Err(String::from("Variables must be a JSON object")),
            Err(error) => return Err(format!("Variables JSON: {error}")),
        };

    let mut context = Context::default();
    variables
        .into_iter()
        .for_each(|(name, value)| context.add_variable_from_value(name, json_value::to_cel(value)));

    Ok(context)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evaluate(expression: &str, variables: &str) -> Result<String, String> {
        Cel::evaluate(expression.to_string(), variables.to_string())
    }

    #[test]
    fn evaluates_arithmetic() {
        assert_eq!(evaluate("1 + 2 * 3", "{}").unwrap(), "7");
    }

    #[test]
    fn evaluates_a_string_result_as_json() {
        assert_eq!(
            evaluate("name + \"!\"", r#"{"name": "Betty"}"#).unwrap(),
            r#""Betty!""#
        );
    }

    #[test]
    fn evaluates_a_boolean_over_nested_variables() {
        let variables = r#"{"user": {"age": 21, "roles": ["admin", "dev"]}}"#;
        assert_eq!(
            evaluate("user.age >= 18 && 'admin' in user.roles", variables).unwrap(),
            "true"
        );
    }

    #[test]
    fn evaluates_list_macros() {
        assert_eq!(
            evaluate(
                "items.map(i, i * 2).filter(i, i > 2)",
                r#"{"items": [1, 2, 3]}"#
            )
            .unwrap(),
            "[4,6]"
        );
    }

    #[test]
    fn evaluates_a_map_result_as_json_object() {
        assert_eq!(
            evaluate("{'total': a + b}", r#"{"a": 1, "b": 2}"#).unwrap(),
            r#"{"total":3}"#
        );
    }

    #[test]
    fn evaluates_null() {
        assert_eq!(evaluate("x", r#"{"x": null}"#).unwrap(), "null");
    }

    #[test]
    fn does_not_support_regex_matches() {
        let error = evaluate("'a'.matches('a')", "{}").unwrap_err();
        assert!(error.starts_with("Evaluation: "), "{error}");
    }

    #[test]
    fn accepts_variables_that_are_an_empty_object() {
        assert_eq!(evaluate("size('abc')", "{}").unwrap(), "3");
    }

    #[test]
    fn rejects_variables_that_are_not_json() {
        let error = evaluate("1", "{ \"a\" }").unwrap_err();
        assert!(error.starts_with("Variables JSON: "), "{error}");
    }

    #[test]
    fn rejects_variables_that_are_not_an_object() {
        assert_eq!(
            evaluate("1", "[1]").unwrap_err(),
            "Variables must be a JSON object"
        );
    }

    #[test]
    fn reports_a_syntax_error() {
        let error = evaluate("1 +", "{}").unwrap_err();
        assert!(error.starts_with("Expression: "), "{error}");
    }

    #[test]
    fn reports_an_undeclared_variable() {
        let error = evaluate("missing + 1", "{}").unwrap_err();
        assert!(error.contains("missing"), "{error}");
    }

    #[test]
    fn reports_an_evaluation_error() {
        let error = evaluate("1 / 0", "{}").unwrap_err();
        assert!(error.starts_with("Evaluation: "), "{error}");
    }
}
