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

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

struct Component;

impl StoreGuest for Component {
    fn store_file(
        helper_context: HelperContext,
        model: Model,
        property: Vec<Property>,
        filename: String,
        data: String,
    ) -> Result<String, String> {
        let property = property
            .first()
            .ok_or(String::from("Failed to fetch file property"))?;

        let file_bytes = BASE64
            .decode(&data)
            .map_err(|error| format!("Failed to decode base64 source: {error}"))?;

        let content_type = mime_guess::from_path(&filename)
            .first_or_octet_stream()
            .to_string();
        let unique_filename = make_unique_filename(&filename);

        let upload_result = upload_file::upload(
            &helper_context,
            &model,
            property,
            &file_bytes,
            &unique_filename,
            &content_type,
        )
        .map_err(|error| format!("Upload failed: {error}"))?;

        Ok(upload_result.reference)
    }
}

fn make_unique_filename(filename: &str) -> String {
    let random_bytes = crate::bindings::wasi::random::random::get_random_bytes(8);
    let hex: String = random_bytes.iter().map(|b| format!("{b:02x}")).collect();

    match filename.rsplit_once('.') {
        Some((stem, ext)) => format!("{stem}_{hex}.{ext}"),
        None => format!("{filename}_{hex}"),
    }
}

bindings::export!(Component with_types_in bindings);
