use crate::exports::betty_blocks::split_text::split_text::Guest;

wit_bindgen::generate!({ generate_all });

struct SplitText;

impl Guest for SplitText {
    fn split_all(string: String, split_by: String) -> Vec<String> {
        string.split(&split_by).map(String::from).collect()
    }

    /* Commented out until tuples are supported.
    fn split_once(string: String, split_by: String) -> Option<(String, String)> {
        string
            .split_once(&split_by)
            .map(|tuple| (String::from(tuple.0), String::from(tuple.1)))
    }
    */
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

/* Commented out until tuples are supported.
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
*/

/// Proptests have to be run as unit tests, because integration tests on cdylib crates aren't able to directly interact with the crate.
#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn splits_the_correct_amount_of_times_with_random_input(
            string in ".{0,100}",
            separator in ".{1,10}",
        ) {
            let result = SplitText::split_all(string.clone(), separator.clone());

            let expected_count = string.matches(&separator).count() + 1;
            prop_assert_eq!(result.len(), expected_count);

            for element in &result {
                prop_assert!(
                    !element.contains(&separator),
                    "element {:?} still contains separator {:?}",
                    element,
                    separator,
                );
            }
        }
    }
}
