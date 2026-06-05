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
        fn splits_the_correct_amount_of_times_with_separator_from_string(
            string in ".{2,100}",
            separator_start in 0usize..100,
            separator_length in 1usize..10,
        ) {
            let character_count = string.chars().count();
            prop_assume!(character_count > separator_length);

            let start = separator_start % (character_count - separator_length);
            let separator: String = string.chars().skip(start).take(separator_length).collect();

            let result = SplitText::split_all(string.clone(), separator.clone());

            let expected_count = string.matches(separator.as_str()).count() + 1;
            prop_assert_eq!(result.len(), expected_count);

            for element in &result {
                prop_assert!(
                    !element.contains(separator.as_str()),
                    "element {:?} still contains separator {:?}",
                    element,
                    separator,
                );
            }
        }

        #[test]
        fn returns_the_original_string_when_separator_is_absent(
            string in ".{1,100}",
            separator in ".{1,10}",
        ) {
            prop_assume!(!string.contains(&separator));

            let result = SplitText::split_all(string.clone(), separator.clone());

            prop_assert_eq!(result.len(), 1);
            prop_assert_eq!(&result[0], &string);
        }
    }
}
