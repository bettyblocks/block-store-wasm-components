pub mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::StoreFileBase64;
    export!(StoreFileBase64);
}

use bindings::{
    betty_blocks_types::data_api::data_api::HelperContext,
    betty_blocks_types::types::types::BettyProperty,
    betty_blocks_types::upload_file::upload_file,
    exports::betty_blocks::store_file_base64::store_base64::{BettyModel, Guest as StoreGuest},
};

struct StoreFileBase64;

impl StoreGuest for StoreFileBase64 {
    fn store_file(
        helper_context: HelperContext,
        model: BettyModel,
        property: Vec<BettyProperty>,
        data: String,
        filename: String,
        file_extension: String,
    ) -> Result<String, String> {
        if string_is_empty(&filename) {
            return Err(String::from("Filename must be set"));
        }
        if string_is_empty(&file_extension) {
            return Err(String::from("File extension must be set"));
        }

        let property = property
            .into_iter()
            .next()
            .ok_or("Failed to fetch file property")?;

        let full_filename = if file_extension.starts_with('.') {
            format!("{filename}{file_extension}")
        } else {
            let file_extension = file_extension.to_lowercase();
            format!("{filename}.{file_extension}")
        };

        let upload_result = upload_file::upload(
            &helper_context,
            &upload_file::Input {
                model,
                property,
                file_base64: data,
                full_filename,
            },
        )
        .map_err(|error| format!("Upload failed: {error}"))?;

        Ok(upload_result.reference)
    }
}

/// Checks if the string is empty or only whitespace.
fn string_is_empty(string: &str) -> bool {
    string.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_string_is_invalid() {
        assert!(string_is_empty(""));
    }

    #[test]
    fn only_whitespace_in_string_is_invalid() {
        assert!(string_is_empty("   "));
    }

    #[test]
    fn not_empty_string_is_valid() {
        assert!(!string_is_empty("jpg"));
    }
}
