pub mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::GenerateUuid;
    export!(GenerateUuid);
}

use crate::bindings::exports::betty_blocks::generate_uuid::generate_uuid::Guest;

pub struct GenerateUuid;

impl Guest for GenerateUuid {
    fn generate_uuid() -> String {
        String::from(uuid::Uuid::new_v4())
    }
}

#[test]
fn is_valid_uuidv4() {
    let uuid = GenerateUuid::generate_uuid();

    assert_eq!(uuid.len(), 32 + 4);

    for (index, character) in uuid.chars().enumerate() {
        if index == 8 || index == 8 + 5 || index == 8 + 5 + 5 || index == 8 + 5 + 5 + 5 {
            assert_eq!(character, '-');
        } else if index == 14 {
            assert_eq!(character, '4');
        } else {
            assert!(character.is_ascii_hexdigit());
        }
    }
}
