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
        check_filename_and_extension(&filename, &file_extension)?;

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

        if cfg!(not(test)) {
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
        } else {
            Ok(String::default())
        }
    }
}

/// Checks if the filename and file_extension aren't empty.
fn check_filename_and_extension(filename: &str, file_extension: &str) -> Result<(), &'static str> {
    if filename.trim().is_empty() {
        return Err("Filename must be set");
    }
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
        assert!(check_filename_and_extension("", "jpg").is_err());
    }

    #[test]
    fn empty_file_extension_is_invalid() {
        assert!(check_filename_and_extension("some_filename", "").is_err());
    }

    #[test]
    fn only_whitespace_filename_is_invalid() {
        assert!(check_filename_and_extension("   ", "jpg").is_err());
    }

    #[test]
    fn only_whitespace_file_extension_is_invalid() {
        assert!(check_filename_and_extension("some_filename", "   ").is_err());
    }

    #[test]
    fn valid_filename_and_extension_is_ok() {
        assert!(check_filename_and_extension("some_filename", "jpg").is_ok());
    }

    /// Proptests have to be run as unit tests, because integration tests on cdylib crates aren't able to directly interact with the crate.
    mod proptests {
        use super::*;
        use proptest::prelude::*;


    fn get_non_testable_params() -> (HelperContext, Model, Vec<Property>) {
        let helper_context = HelperContext {
            action_id: String::default(),
            application_id: String::default(),
            encrypted_configurations: None,
            jwt: None,
            log_id: String::default(),
        };
        let model = Model {
            name: String::default(),
        };
        let property = vec![Property {
            name: String::default(),
        }];

        (helper_context, model, property)
    }


        proptest! {
            #[test]
            fn empty_or_whitespace_filename_is_invalid(
                filename in r"\s{0,10}",
                file_extension in ".{1,10}",
            ) {
                let (helper_context, model, property) = get_non_testable_params();

                let result = Component::store_file(
                    helper_context,
                    model,
                    property,
                    String::from("SGVsbG8sIFdvcmxkIGRhZmo7a2RzYWpmbGtzYWpmIQo="),
                    filename,
                    file_extension,
                );

                prop_assert!(result.is_err());
            }

            #[test]
            fn empty_or_whitespace_file_extension_is_invalid(
                filename in ".{1,10}",
                file_extension in r"\s{0,10}",
            ) {
                let (helper_context, model, property) = get_non_testable_params();

                let result = Component::store_file(
                    helper_context,
                    model,
                    property,
                    String::from("SGVsbG8sIFdvcmxkIGRhZmo7a2RzYWpmbGtzYWpmIQo="),
                    filename,
                    file_extension,
                );

                prop_assert!(result.is_err());
            }

            #[test]
            fn non_whitespace_filename_and_extension_is_valid(
                filename in r"\S.{0,9}",
                file_extension in r"\S.{0,9}",
            ) {
                let (helper_context, model, property) = get_non_testable_params();

                let result = Component::store_file(
                    helper_context,
                    model,
                    property,
                    String::from("SGVsbG8sIFdvcmxkIGRhZmo7a2RzYWpmbGtzYWpmIQo="),
                    filename,
                    file_extension,
                );

                prop_assert!(result.is_ok());
            }
        }
    }
}
