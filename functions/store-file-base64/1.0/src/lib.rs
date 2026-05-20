pub mod bindings {
    wit_bindgen::generate!({
        generate_all,
    });
}

use bindings::{
    betty_blocks::data_api::data_api::HelperContext,
    betty_blocks::file::upload_file,
    betty_blocks::types::types::Property,
    exports::betty_blocks::file::store_base64::{Guest as StoreGuest, Model},
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
        let property = property
            .first()
            .ok_or(String::from("Failed to fetch file property"))?;

        let file_bytes = BASE64
            .decode(&data)
            .map_err(|error| format!("Failed to decode base64 source: {error}"))?;

        let full_filename = if file_extension.starts_with('.') {
            let file_extension = file_extension.to_lowercase();
            format!("{filename}{file_extension}")
        } else {
            format!("{filename}.{file_extension}")
        };

        let upload_result = upload_file::upload(
            &helper_context,
            &model,
            property,
            &file_bytes,
            &full_filename,
        )
        .map_err(|error| format!("Upload failed: {error}"))?;

        Ok(upload_result.reference)
    }
}

bindings::export!(Component with_types_in bindings);
