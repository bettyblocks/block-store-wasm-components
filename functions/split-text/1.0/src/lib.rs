use crate::exports::betty_blocks::split_text::split_text::Guest;

wit_bindgen::generate!({ generate_all });

struct SplitText;

impl Guest for SplitText {
    fn split_all(string: String, split_by: String) -> Vec<String> {
        string.split(&split_by).map(String::from).collect()
    }

    fn split_once(string: String, split_by: String) -> Option<(String, String)> {
        string
            .split_once(&split_by)
            .map(|tuple| (String::from(tuple.0), String::from(tuple.1)))
    }
}

export! {SplitText}

#[test]
fn split_all_splits_the_correct_amount_of_times() {
    let result = SplitText::split_all(String::from("hi hi hi hi"), String::from(" "));
    assert_eq!(result, vec!["hi", "hi", "hi", "hi"]);
}

#[test]
fn can_split_all_by_string() {
    let result = SplitText::split_all(
        String::from("hi something_large hi"),
        String::from(" something_large "),
    );
    assert_eq!(result, vec!["hi", "hi"]);
}

#[test]
fn invalid_split_all_gives_one_item() {
    let result = SplitText::split_all(
        String::from("hi hi hi hi"),
        String::from(" something_large "),
    );
    assert_eq!(result, vec!["hi hi hi hi"]);
}

#[test]
fn split_once_only_splits_once() {
    let result = SplitText::split_once(String::from("hi hi hi hi"), String::from(" "));
    assert_eq!(result, Some((String::from("hi"), String::from("hi hi hi"))));
}

#[test]
fn invalid_split_once_gives_none() {
    let result = SplitText::split_once(
        String::from("hi hi hi hi"),
        String::from(" something_large "),
    );
    assert_eq!(result, None);
}
