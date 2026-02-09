use crate::exports::betty_blocks::concat_text::concat_text::Guest;

wit_bindgen::generate!({ generate_all });

struct ConcatText;

impl Guest for ConcatText {
    fn concat_strings(strings: Vec<String>) -> String {
        strings.join("")
    }

    fn concat_strings_with_separator(strings: Vec<String>, separator: String) -> String {
        strings.join(&separator)
    }
}

export! {ConcatText}

#[test]
fn can_concat_a_list_of_strings() {
    let result = ConcatText::concat_strings(
        vec!["hi", "hi", "hi", "hi"]
            .into_iter()
            .map(String::from)
            .collect(),
    );
    assert_eq!(result, String::from("hihihihi"),);
}

#[test]
fn can_concat_a_list_of_strings_with_a_separator() {
    let result = ConcatText::concat_strings_with_separator(
        vec!["hi", "hi", "hi", "hi"]
            .into_iter()
            .map(String::from)
            .collect(),
        String::from(" "),
    );
    assert_eq!(result, String::from("hi hi hi hi"),);
}
