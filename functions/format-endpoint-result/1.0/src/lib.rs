use crate::exports::betty_blocks::format_endpoint_result::format_endpoint_result::{
    Guest, Header, JsonString,
};

wit_bindgen::generate!({ generate_all });

#[derive(Debug, Clone, serde::Serialize)]
#[cfg_attr(test, derive(serde::Deserialize))]
struct FormattedEndpointResult {
    #[serde(rename = "statusCode")]
    status_code: u16,
    /// The body is explicitly left unparsed (and so will be serialized as a string within a json string) to be compatible with the old javascript action.
    body: String,
    headers: Vec<(String, serde_json::Value)>,
}

struct Component;

impl Guest for Component {
    fn format_endpoint_result(
        status_code: u16,
        body: JsonString,
        headers: Vec<Header>,
    ) -> Result<JsonString, String> {
        let mut parsed_headers = Vec::new();

        for header in headers {
            parsed_headers.push((
                header.key,
                serde_json::from_str(&header.value).map_err(|_| {
                    String::from("Could not serialize formatted endpoint result to JSON")
                })?,
            ))
        }

        serde_json::to_string(&FormattedEndpointResult {
            status_code,
            body,
            headers: parsed_headers,
        })
        .map_err(|_| String::from("Could not serialize formatted endpoint result to JSON"))
    }
}

export! {Component}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_result_is_correctly_formatted_test() {
        let status = 200;
        let body = String::from("{\"result\": true}");
        let headers = vec![Header {
            key: String::from("Accept"),
            value: String::from("\"application/json\""),
        }];

        let formatted_endpoint_result: FormattedEndpointResult = serde_json::from_str(
            &Component::format_endpoint_result(status, body.clone(), headers.clone()).unwrap(),
        )
        .unwrap();

        assert_eq!(formatted_endpoint_result.status_code, status);
        assert_eq!(formatted_endpoint_result.body, body);
        assert_eq!(
            formatted_endpoint_result.headers,
            vec![(
                headers[0].key.clone(),
                serde_json::from_str(&headers[0].value).unwrap()
            )]
        );
    }
}
