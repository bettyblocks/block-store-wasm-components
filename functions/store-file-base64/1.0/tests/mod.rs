mod bindings {
    wasmtime_testing_helper::bindgen!("store-file-base64");

    wasmtime_testing_helper::setup!(StoreFileBase64);
}

use std::sync::{Arc, Mutex};

use bindings::betty_blocks_types::data_api::data_api::HelperContext;
use bindings::betty_blocks_types::types::types::{BettyModel, BettyProperty};
use bindings::betty_blocks_types::upload_file::upload_file::{Input, UploadResult};

const UPLOAD_FILE: &str = "betty-blocks-types:upload-file/upload-file@4.0.0";

/// A 1x1 PNG, as the platform would pass it in the data option.
const TEST_BASE64: &str =
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=";

type Uploads = Arc<Mutex<Vec<(HelperContext, Input)>>>;

fn test_helper_context() -> HelperContext {
    HelperContext {
        application_id: String::from("app-id"),
        action_id: String::from("action-id"),
        log_id: String::from("log-id"),
        encrypted_configurations: None,
        jwt: Some(String::from("jwt")),
    }
}

fn document_model() -> BettyModel {
    BettyModel {
        name: String::from("Document"),
    }
}

fn file_property() -> Vec<BettyProperty> {
    vec![BettyProperty {
        name: String::from("file"),
    }]
}

/// Mocks upload-file, recording every input it receives and answering with `response`.
fn setup_harness(
    response: Result<UploadResult, String>,
) -> (
    wasmtime_testing_helper::ComponentCompositionBuilder,
    Uploads,
) {
    let uploads: Uploads = Arc::default();
    let recorded = Arc::clone(&uploads);
    let mut harness = bindings::harness();
    harness.mock(
        UPLOAD_FILE,
        "upload",
        move |_context, (helper_context, input): (HelperContext, Input)| {
            recorded.lock().unwrap().push((helper_context, input));
            Ok((response.clone(),))
        },
    );
    (harness, uploads)
}

fn successful_upload() -> Result<UploadResult, String> {
    Ok(UploadResult {
        reference: String::from("file-reference"),
        file_size: 70,
        message: None,
    })
}

fn store_file(
    response: Result<UploadResult, String>,
    property: &[BettyProperty],
    data: &str,
    filename: &str,
    file_extension: &str,
) -> (Result<String, String>, Vec<(HelperContext, Input)>) {
    let (harness, uploads) = setup_harness(response);
    let mut component = bindings::instantiate(harness);
    let result = component
        .component
        .betty_blocks_store_file_base64_store_base64()
        .call_store_file(
            &mut component.store,
            &test_helper_context(),
            &document_model(),
            property,
            data,
            filename,
            file_extension,
        )
        .unwrap();
    let uploads = uploads.lock().unwrap().clone();
    assert_eq!(component.call_count(UPLOAD_FILE, "upload"), uploads.len());
    (result, uploads)
}

#[test]
fn passes_base64_to_upload_file_unchanged() {
    let (result, uploads) = store_file(
        successful_upload(),
        &file_property(),
        TEST_BASE64,
        "testfile",
        ".png",
    );

    assert_eq!(result, Ok(String::from("file-reference")));
    assert_eq!(uploads.len(), 1);
    let (helper_context, input) = &uploads[0];
    assert_eq!(helper_context.application_id, "app-id");
    assert_eq!(helper_context.action_id, "action-id");
    assert_eq!(helper_context.log_id, "log-id");
    assert_eq!(helper_context.jwt.as_deref(), Some("jwt"));
    assert_eq!(input.file_base64, TEST_BASE64);
    assert_eq!(input.full_filename, "testfile.png");
    assert_eq!(input.model.name, "Document");
    assert_eq!(input.property.name, "file");
}

#[test]
fn lowercases_extension_without_dot() {
    let (result, uploads) = store_file(
        successful_upload(),
        &file_property(),
        TEST_BASE64,
        "testfile",
        "PNG",
    );

    assert_eq!(result, Ok(String::from("file-reference")));
    assert_eq!(uploads[0].1.full_filename, "testfile.png");
}

#[test]
fn uses_first_property() {
    let property = [
        BettyProperty {
            name: String::from("first"),
        },
        BettyProperty {
            name: String::from("second"),
        },
    ];
    let (_, uploads) = store_file(
        successful_upload(),
        &property,
        TEST_BASE64,
        "testfile",
        ".png",
    );

    assert_eq!(uploads[0].1.property.name, "first");
}

#[test]
fn returns_upload_file_error() {
    let (result, _) = store_file(
        Err(String::from("GraphQL errors: forbidden")),
        &file_property(),
        TEST_BASE64,
        "testfile",
        ".png",
    );

    assert_eq!(
        result,
        Err(String::from("Upload failed: GraphQL errors: forbidden"))
    );
}

#[test]
fn fails_with_empty_filename() {
    let (result, uploads) = store_file(
        successful_upload(),
        &file_property(),
        TEST_BASE64,
        "",
        ".png",
    );

    assert_eq!(result, Err(String::from("Filename must be set")));
    assert!(uploads.is_empty());
}

#[test]
fn fails_with_whitespace_only_filename() {
    let (result, uploads) = store_file(
        successful_upload(),
        &file_property(),
        TEST_BASE64,
        "   ",
        ".png",
    );

    assert_eq!(result, Err(String::from("Filename must be set")));
    assert!(uploads.is_empty());
}

#[test]
fn fails_with_empty_file_extension() {
    let (result, uploads) = store_file(
        successful_upload(),
        &file_property(),
        TEST_BASE64,
        "testfile",
        "",
    );

    assert_eq!(result, Err(String::from("File extension must be set")));
    assert!(uploads.is_empty());
}

#[test]
fn fails_with_empty_property_list() {
    let (result, uploads) = store_file(successful_upload(), &[], TEST_BASE64, "testfile", ".png");

    assert_eq!(result, Err(String::from("Failed to fetch file property")));
    assert!(uploads.is_empty());
}
