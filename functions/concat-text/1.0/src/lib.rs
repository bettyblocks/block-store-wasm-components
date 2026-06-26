mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::ConcatText;
    export!(ConcatText);
}

use crate::bindings::exports::betty_blocks::concat_text::concat_text::Guest;

struct ConcatText;

impl Guest for ConcatText {
    fn concat_text(first_text: String, second_text: String) -> String {
        format!("{}{}", first_text, second_text)
    }

    fn concat_text_with_separator(
        first_text: String,
        second_text: String,
        separator: String,
    ) -> String {
        format!("{}{}{}", first_text, separator, second_text)
    }

    fn concat_text_list(text_list: Vec<String>) -> String {
        text_list.join("")
    }

    fn concat_text_list_with_separator(text_list: Vec<String>, separator: String) -> String {
        text_list.join(&separator)
    }
}

#[test]
fn can_concat_two_strings() {
    let result = ConcatText::concat_text(String::from("hi"), String::from("ha"));
    assert_eq!(result, String::from("hiha"));
}

#[test]
fn can_concat_two_strings_with_separator() {
    let result = ConcatText::concat_text_with_separator(
        String::from("hi"),
        String::from("ho"),
        String::from(" "),
    );
    assert_eq!(result, String::from("hi ho"));
}

#[test]
fn can_concat_a_list_of_strings() {
    let result = ConcatText::concat_text_list(
        vec!["hi", "123", "ha", "ho"]
            .into_iter()
            .map(String::from)
            .collect(),
    );
    assert_eq!(result, String::from("hi123haho"),);
}

#[test]
fn can_concat_a_list_of_strings_with_a_separator() {
    let result = ConcatText::concat_text_list_with_separator(
        vec!["fi", "oa", "30", "22"]
            .into_iter()
            .map(String::from)
            .collect(),
        String::from(" "),
    );
    assert_eq!(result, String::from("fi oa 30 22"),);
}
