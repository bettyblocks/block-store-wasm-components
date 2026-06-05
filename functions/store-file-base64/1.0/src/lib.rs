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
        check_filename(&filename)?;
        check_file_extension(&file_extension)?;

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

/// Checks if the filename isn't empty or only whitespace.
fn check_filename(filename: &str) -> Result<(), &'static str> {
    if filename.trim().is_empty() {
        return Err("Filename must be set");
    }

    Ok(())
}

/// Checks if the file extension isn't empty or only whitespace.
fn check_file_extension(file_extension: &str) -> Result<(), &'static str> {
    if file_extension.trim().is_empty() {
        return Err("File extension must be set");
    }

    Ok(())
}

bindings::export!(Component with_types_in bindings);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_filename_is_invalid() {
        assert!(check_filename("").is_err());
    }

    #[test]
    fn only_whitespace_filename_is_invalid() {
        assert!(check_filename("   ").is_err());
    }

    #[test]
    fn empty_file_extension_is_invalid() {
        assert!(check_file_extension("").is_err());
    }

    #[test]
    fn only_whitespace_file_extension_is_invalid() {
        assert!(check_file_extension("   ").is_err());
    }

    #[test]
    fn valid_filename_is_ok() {
        assert!(check_filename("some_filename").is_ok());
    }

    #[test]
    fn valid_file_extension_is_ok() {
        assert!(check_file_extension("jpg").is_ok());
    }
}
