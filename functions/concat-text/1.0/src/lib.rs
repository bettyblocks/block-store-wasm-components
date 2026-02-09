use crate::exports::betty_blocks::concat_text::concat_text::Guest;

wit_bindgen::generate!({ generate_all });

struct ConcatText;

impl Guest for ConcatText {
    fn concat_strings(first_string: String, second_string: String) -> String {
        format!("{}{}", first_string, second_string)
    }

    fn concat_strings_with_separator(first_string: String, second_string: String, separator: String) -> String {
        format!("{}{}{}", first_string, separator, second_string)
    }

    fn concat_string_list(strings: Vec<String>) -> String {
        strings.join("")
    }

    fn concat_string_list_with_separator(strings: Vec<String>, separator: String) -> String {
        strings.join(&separator)
    }
}

export! {ConcatText}

#[test]
fn can_concat_two_strings() {
    let result = ConcatText::concat_strings(
        String::from("hi"),
        String::from("ha"),
    );
    assert_eq!(result, String::from("hiha"));
}

#[test]
fn can_concat_two_strings_with_separator() {
    let result = ConcatText::concat_strings_with_separator(
        String::from("hi"),
        String::from("ho"),
        String::from(" "),
    );
    assert_eq!(result, String::from("hi ho"));
}

#[test]
fn can_concat_a_list_of_strings() {
    let result = ConcatText::concat_string_list(
        vec!["hi", "123", "ha", "ho"]
            .into_iter()
            .map(String::from)
            .collect(),
    );
    assert_eq!(result, String::from("hi123haho"),);
}

#[test]
fn can_concat_a_list_of_strings_with_a_separator() {
    let result = ConcatText::concat_string_list_with_separator(
        vec!["fi", "oa", "30", "22"]
            .into_iter()
            .map(String::from)
            .collect(),
        String::from(" "),
    );
    assert_eq!(result, String::from("fi oa 30 22"),);
}
