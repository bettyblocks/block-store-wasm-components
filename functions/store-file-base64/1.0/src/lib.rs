pub mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::StoreFileBase64;
    export!(StoreFileBase64);
}

use bindings::{
    betty_blocks_types::data_api::data_api::HelperContext as UploadHelperContext,
    betty_blocks_types::types::types::{BettyModel, BettyProperty},
    betty_blocks_types::upload_file::upload_file,
    betty_blocks_utilities::data_api::data_api::HelperContext,
    betty_blocks_utilities::types::types::Property,
    exports::betty_blocks::store_file_base64::store_base64::{Guest as StoreGuest, Model},
};

struct StoreFileBase64;

impl StoreGuest for StoreFileBase64 {
    fn store_file(
        helper_context: HelperContext,
        model: Model,
        property: Vec<Property>,
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

        // upload-file@3.0.0 lives under a different WIT package (betty-blocks-types)
        // than our own exported interface (betty-blocks-utilities), so its
        // identically-shaped record types are distinct Rust types requiring
        // a field-by-field conversion.
        let upload_helper_context = UploadHelperContext {
            application_id: helper_context.application_id,
            action_id: helper_context.action_id,
            log_id: helper_context.log_id,
            encrypted_configurations: helper_context.encrypted_configurations,
            jwt: helper_context.jwt,
        };
        let model = BettyModel { name: model.name };
        let property = BettyProperty {
            name: property.name,
        };

        let upload_result = upload_file::upload(
            &upload_helper_context,
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
