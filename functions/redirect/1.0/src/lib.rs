use crate::exports::betty_blocks::redirect::redirect::{Guest, Output};

wit_bindgen::generate!({ generate_all });

struct Component;

impl Guest for Component {
    fn redirect(redirect_url: String) -> Output {
        Output {
            status_code: 302,
            body: String::from("Redirect"),
            headers: vec![(String::from("Location"), redirect_url)],
        }
    }
}

export! {Component}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redirect_points_to_correct_url() {
        let url = String::from("http://example.com");

        assert_eq!(Component::redirect(url.clone()).headers[0].1, url);
    }
}
