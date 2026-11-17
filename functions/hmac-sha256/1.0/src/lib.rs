use hmac::{Hmac, Mac};
use sha2::Sha256;

mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::HmacSha256;
    export!(HmacSha256);
}

use crate::bindings::exports::betty_blocks::hmac_sha256::hmac_sha256::Guest;

struct HmacSha256;

impl Guest for HmacSha256 {
    /// Returns the lowercase hex encoded HMAC-SHA256 of `message` signed with `key`.
    fn hmac_sha256(key: String, message: String) -> String {
        // HMAC accepts keys of any length, so this never fails.
        let mut mac =
            Hmac::<Sha256>::new_from_slice(key.as_bytes()).expect("HMAC accepts keys of any size");
        mac.update(message.as_bytes());
        mac.finalize()
            .into_bytes()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // RFC 4231 test case 2
    #[test]
    fn matches_rfc_4231_vector() {
        let result = HmacSha256::hmac_sha256(
            String::from("Jefe"),
            String::from("what do ya want for nothing?"),
        );
        assert_eq!(
            result,
            "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
        );
    }

    #[test]
    fn handles_keys_longer_than_block_size() {
        let result = HmacSha256::hmac_sha256("a".repeat(200), String::from("message"));
        assert_eq!(result.len(), 64);
    }

    #[test]
    fn different_keys_give_different_results() {
        let first = HmacSha256::hmac_sha256(String::from("one"), String::from("message"));
        let second = HmacSha256::hmac_sha256(String::from("two"), String::from("message"));
        assert_ne!(first, second);
    }
}
