use crate::exports::betty_blocks::concat_text::concat_text::Guest;

wit_bindgen::generate!({ generate_all });

struct ConcatText;

impl Guest for ConcatText {
    fn concat_text(first_string: String, second_string: String) -> String {
        String::default()
    }
}

export! {ConcatText}

