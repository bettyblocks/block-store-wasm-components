use crate::exports::betty_blocks::format_endpoint_result::format_endpoint_result::{
    Guest, Header, JsonString, Output,
};

wit_bindgen::generate!({ generate_all });

struct Component;

impl Guest for Component {
    fn format_endpoint_result(status_code: u16, body: JsonString, headers: Vec<Header>) -> Output {
        Output {
            status_code,
            body,
            headers: headers
                .into_iter()
                .map(|header| (header.key, header.value))
                .collect(),
        }
    }
}

export! {Component}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_result_is_correctly_formatted_test() {
        let status = 200;
        let body = String::from("{result: true}");
        let headers = vec![Header {
            key: String::from("Accept"),
            value: String::from("application/json"),
        }];

        let formatted_endpoint_result =
            Component::format_endpoint_result(status, body.clone(), headers.clone());

        assert_eq!(formatted_endpoint_result.status_code, status);
        assert_eq!(formatted_endpoint_result.body, body);
        assert_eq!(
            formatted_endpoint_result.headers,
            vec![(headers[0].key.clone(), headers[0].value.clone())]
        );
    }
}
