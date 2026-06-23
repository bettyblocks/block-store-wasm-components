mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::RedirectUrl;
    export!(RedirectUrl);
}

use crate::bindings::exports::betty_blocks::redirect_url::redirect_url::{Guest, JsonString};

#[derive(Debug, Clone, serde::Serialize)]
#[cfg_attr(test, derive(serde::Deserialize))]
struct FormattedEndpointResult {
    #[serde(rename = "statusCode")]
    status_code: u16,
    body: String,
    headers: Vec<(String, String)>,
}

struct RedirectUrl;

impl Guest for RedirectUrl {
    fn redirect_url(redirect_url: String) -> Result<JsonString, String> {
        serde_json::to_string(&FormattedEndpointResult {
            status_code: 302,
            body: String::from("Redirect"),
            headers: vec![(String::from("Location"), redirect_url)],
        })
        .map_err(|_error| String::from("Could not serialize redirect response as JSON"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redirect_points_to_correct_url() {
        let url = String::from("http://example.com");

        assert_eq!(
            serde_json::from_str::<FormattedEndpointResult>(
                &RedirectUrl::redirect_url(url.clone()).unwrap()
            )
            .unwrap()
            .headers[0]
                .1,
            url
        );
    }
}
