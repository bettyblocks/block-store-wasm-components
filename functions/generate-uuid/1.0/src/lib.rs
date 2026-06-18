use std::cell::RefCell;

pub mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::GenerateUuid;
    export!(GenerateUuid);
}
use crate::bindings::betty_blocks::random_hex::random_hex::generate_random_hex;
use crate::bindings::exports::betty_blocks::generate_uuid::generate_uuid::{
    Guest, GuestUuidBatch,
};

pub struct GenerateUuid;

impl Guest for GenerateUuid {
    type UuidBatch = UuidBatchState;

    fn generate_uuid() -> String {
        let uuid = make_the_uuid();
        let random_hex = generate_random_hex(8);
        format!("{uuid}-{random_hex}")
    }
}

pub struct UuidBatchState {
    generated: RefCell<Vec<String>>,
}

impl GuestUuidBatch for UuidBatchState {
    fn new() -> Self {
        UuidBatchState {
            generated: RefCell::new(Vec::new()),
        }
    }

    fn generate_next(&self) -> String {
        let uuid = make_the_uuid();
        let random_hex = generate_random_hex(8);
        let result = format!("{uuid}-{random_hex}");
        self.generated.borrow_mut().push(result.clone());
        result
    }

    fn count(&self) -> u32 {
        self.generated.borrow().len() as u32
    }

    fn collect(&self) -> Vec<String> {
        self.generated.borrow().clone()
    }
}

fn make_the_uuid() -> String {
    String::from(uuid::Uuid::new_v4())
}

#[test]
fn is_uuidv4_valid() {
    let result = make_the_uuid();

    assert_eq!(result.len(), 36);

    for (index, character) in result.chars().enumerate() {
        if index == 8 || index == 13 || index == 18 || index == 23 {
            assert_eq!(character, '-');
        } else if index == 14 {
            assert_eq!(character, '4');
        } else {
            assert!(character.is_ascii_hexdigit());
        }
    }
}
