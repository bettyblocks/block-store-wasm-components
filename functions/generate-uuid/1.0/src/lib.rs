#[cfg(not(test))]
pub mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::GenerateUuid;
    export!(GenerateUuid);
}
#[cfg(not(test))]
use crate::bindings::betty_blocks::random_hex::random_hex::generate_random_hex;
#[cfg(not(test))]
use crate::bindings::exports::betty_blocks::generate_uuid::generate_uuid::Guest;

#[cfg(test)]
mod bindings {
    pub trait Guest {
        fn generate_uuid() -> String;
    }
}
#[cfg(test)]
use crate::bindings::Guest;
#[cfg(test)]
fn generate_random_hex(_size: u32) -> String {
    String::from("AABBCCDD")
}

pub struct GenerateUuid;

impl Guest for GenerateUuid {
    fn generate_uuid() -> String {
        let uuid = String::from(uuid::Uuid::new_v4());
        let random_hex = generate_random_hex(8);
        format!("{uuid}-{random_hex}")
    }
}

#[test]
fn is_uuidv4_with_random_hex_valid() {
    let result = GenerateUuid::generate_uuid();

    // Expected format: <uuid>-<random-hex>
    let parts: Vec<&str> = result.rsplitn(2, '-').collect();
    let random_hex_part = parts[0];
    let uuid_part = parts[1];

    assert_eq!(random_hex_part, "AABBCCDD");
    assert_eq!(uuid_part.len(), 36);

    for (index, character) in uuid_part.chars().enumerate() {
        if index == 8 || index == 13 || index == 18 || index == 23 {
            assert_eq!(character, '-');
        } else if index == 14 {
            assert_eq!(character, '4');
        } else {
            assert!(character.is_ascii_hexdigit());
        }
    }
}
