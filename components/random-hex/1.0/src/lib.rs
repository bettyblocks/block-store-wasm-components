use rand::Rng;

use crate::exports::betty_blocks::random_hex::random_hex::Guest;

wit_bindgen::generate!({ generate_all });

struct RandomHex;

impl Guest for RandomHex {
    fn generate_random_hex(size: u32) -> String {
        let mut rng_generator = rand::rng();
        let hex_generator_closure = || format!("{:X}", rng_generator.random_range(0..16));
        let hex_iterator = std::iter::repeat_with(hex_generator_closure).take(size as usize);
        hex_iterator.collect()
    }
}

export!(RandomHex);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn randomness_test() {
        let hex1 = RandomHex::generate_random_hex(1000);
        let hex2 = RandomHex::generate_random_hex(1000);
        assert_ne!(hex1, hex2)
    }

    #[test]
    fn length_test() {
        let hex = RandomHex::generate_random_hex(1000);
        assert_eq!(hex.len(), 1000)
    }

    #[test]
    fn content_validity_test() {
        let hex = RandomHex::generate_random_hex(32);
        assert!(u128::from_str_radix(&hex, 16).is_ok());
    }
}
