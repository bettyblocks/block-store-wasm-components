mod bindings {
    wasmtime_testing_helper::bindgen!("store-file-base64");

    wasmtime_testing_helper::setup!(StoreFileBase64);
}

use bindings::betty_blocks_types::data_api::data_api::HelperContext as UploadHelperContext;
use bindings::betty_blocks_types::upload_file::upload_file::{Input, UploadResult};
use bindings::betty_blocks_utilities::data_api::data_api::HelperContext;
use bindings::betty_blocks_utilities::types::types::{Model, Property};

const TEST_BASE64: &str = "dGVzdA==";

fn test_helper_context() -> HelperContext {
    HelperContext {
        application_id: String::from("app-id"),
        action_id: String::from("action-id"),
        log_id: String::from("log-id"),
        encrypted_configurations: None,
        jwt: None,
    }
}

fn setup_harness() -> wasmtime_testing_helper::ComponentCompositionBuilder {
    let mut harness = bindings::harness();
    harness
        .mock(
            "betty-blocks-utilities:data-api/data-api",
            "request",
            |_ctx, _: (HelperContext, String, String)| {
                Ok((Ok::<String, String>(String::from("{}")),))
            },
        )
        .mock(
            "betty-blocks-types:upload-file/upload-file@3.0.0",
            "upload",
            |_ctx, _: (UploadHelperContext, Input)| {
                Ok((Ok::<UploadResult, String>(UploadResult {
                    reference: String::from("file-reference"),
                    file_size: 4,
                    message: None,
                }),))
            },
        );
    harness
}

#[test]
fn stores_file_with_dot_extension() {
    let mut component = bindings::instantiate(setup_harness());
    let interface = component
        .component
        .betty_blocks_store_file_base64_store_base64();
    let result = interface
        .call_store_file(
            &mut component.store,
            &test_helper_context(),
            &Model {
                name: String::from("Document"),
            },
            &[Property {
                name: String::from("file"),
            }],
            TEST_BASE64,
            "testfile",
            ".jpg",
        )
        .unwrap();
    assert_eq!(result, Ok(String::from("file-reference")));
}

#[test]
fn stores_file_with_extension_without_dot() {
    let mut component = bindings::instantiate(setup_harness());
    let interface = component
        .component
        .betty_blocks_store_file_base64_store_base64();
    let result = interface
        .call_store_file(
            &mut component.store,
            &test_helper_context(),
            &Model {
                name: String::from("Document"),
            },
            &[Property {
                name: String::from("file"),
            }],
            TEST_BASE64,
            "testfile",
            "jpg",
        )
        .unwrap();
    assert_eq!(result, Ok(String::from("file-reference")));
}

#[test]
fn fails_with_empty_filename() {
    let mut component = bindings::instantiate(setup_harness());
    let interface = component
        .component
        .betty_blocks_store_file_base64_store_base64();
    let result = interface
        .call_store_file(
            &mut component.store,
            &test_helper_context(),
            &Model {
                name: String::from("Document"),
            },
            &[Property {
                name: String::from("file"),
            }],
            TEST_BASE64,
            "",
            ".jpg",
        )
        .unwrap();
    assert_eq!(result, Err(String::from("Filename must be set")));
}

#[test]
fn fails_with_whitespace_only_filename() {
    let mut component = bindings::instantiate(setup_harness());
    let interface = component
        .component
        .betty_blocks_store_file_base64_store_base64();
    let result = interface
        .call_store_file(
            &mut component.store,
            &test_helper_context(),
            &Model {
                name: String::from("Document"),
            },
            &[Property {
                name: String::from("file"),
            }],
            TEST_BASE64,
            "   ",
            ".jpg",
        )
        .unwrap();
    assert_eq!(result, Err(String::from("Filename must be set")));
}

#[test]
fn fails_with_empty_file_extension() {
    let mut component = bindings::instantiate(setup_harness());
    let interface = component
        .component
        .betty_blocks_store_file_base64_store_base64();
    let result = interface
        .call_store_file(
            &mut component.store,
            &test_helper_context(),
            &Model {
                name: String::from("Document"),
            },
            &[Property {
                name: String::from("file"),
            }],
            TEST_BASE64,
            "testfile",
            "",
        )
        .unwrap();
    assert_eq!(result, Err(String::from("File extension must be set")));
}

#[test]
fn fails_with_empty_property_list() {
    let mut component = bindings::instantiate(setup_harness());
    let interface = component
        .component
        .betty_blocks_store_file_base64_store_base64();
    let result = interface
        .call_store_file(
            &mut component.store,
            &test_helper_context(),
            &Model {
                name: String::from("Document"),
            },
            &[],
            TEST_BASE64,
            "testfile",
            ".jpg",
        )
        .unwrap();
    assert_eq!(result, Err(String::from("Failed to fetch file property")));
}
