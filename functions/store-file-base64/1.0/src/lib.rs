pub mod bindings {
    wit_bindgen::generate!({
        generate_all,
    });
}

use bindings::{
    betty_blocks::data_api::data_api::HelperContext,
    betty_blocks::file::upload_file,
    betty_blocks::types::types::Property,
    exports::betty_blocks::store_file_base64::store_base64::{Guest as StoreGuest, Model},
};

use base64::{Engine, engine::general_purpose::STANDARD as BASE64};

struct Component;

impl StoreGuest for Component {
    fn store_file(
        helper_context: HelperContext,
        model: Model,
        property: Vec<Property>,
        data: String,
        filename: String,
        file_extension: String,
    ) -> Result<String, String> {
        if string_is_empty(&filename) { return Err(String::from("Filename must be set")); }
        if string_is_empty(&file_extension) { return Err(String::from("File extension must be set")); }

        let property = property
            .into_iter()
            .next()
            .ok_or("Failed to fetch file property")?;

        let file_bytes = BASE64
            .decode(&data)
            .map_err(|error| format!("Failed to decode base64 source: {error}"))?;

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
                file_bytes,
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

bindings::export!(Component with_types_in bindings);

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
        assert!(string_is_empty("jpg"));
    }
}
