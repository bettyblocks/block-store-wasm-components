#[cfg(not(feature = "test"))]
pub mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::GenerateUuid;
    export!(GenerateUuid);
}
#[cfg(not(feature = "test"))]
use crate::bindings::exports::betty_blocks::generate_uuid::generate_uuid::Guest;

#[cfg(feature = "test")]
pub mod bindings {
    pub trait Guest {
        fn generate_uuid() -> String;
    }
}
#[cfg(feature = "test")]
use crate::bindings::Guest;

pub struct GenerateUuid;

impl Guest for GenerateUuid {
    fn generate_uuid() -> String {
        String::from(uuid::Uuid::new_v4())
    }
}

#[test]
fn is_uuidv4_valid() {
    let uuidv4 = GenerateUuid::generate_uuid();
    assert_eq!(uuidv4.len(), 32 + 4);
    for (index, character) in uuidv4.chars().enumerate() {
        if index == 8 || index == 8 + 5 || index == 8 + 5 + 5 || index == 8 + 5 + 5 + 5 {
            assert_eq!(character, '-');
        } else if index == 14 {
            assert_eq!(character, '4');
        } else {
            assert!(character.is_ascii_hexdigit());
        }
    }
}
