use md5::Digest;

pub struct AdventCoin {}

impl AdventCoin {
    pub fn mine(secret_key: &str, leading_zeros: u32) -> u32 {
        let mut suffix = 1;

        loop {
            let number = format!("{}{}", secret_key, suffix);
            let digest = md5::compute(&number);

            if Self::digest_with_zeros(&digest, leading_zeros) {
                break;
            }
            suffix += 1;
        }

        suffix
    }

    // Check if hash digest contains leading zeros
    fn digest_with_zeros(digest: &Digest, leading_zeros: u32) -> bool {
        match leading_zeros {
            5 => digest[0] == 0 && digest[1] == 0 && digest[2] & 0xF0 == 0,
            6 => digest[0] == 0 && digest[1] == 0 && digest[2] == 0,
            _ => panic!("Unsupported leading zeros {}", leading_zeros),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Long running test
    fn test_mine() {
        assert_eq!(AdventCoin::mine("abcdef", 5), 609043);
        assert_eq!(AdventCoin::mine("pqrstuv", 5), 1048970);
    }

    #[test]
    fn test_digest_with_zeros() {
        assert!(AdventCoin::digest_with_zeros(
            &md5::compute("abcdef609043"),
            5
        ));
        assert!(AdventCoin::digest_with_zeros(
            &md5::compute("pqrstuv1048970"),
            5
        ));
    }

    #[test]
    fn test_digest_with_non_zeros() {
        assert!(!AdventCoin::digest_with_zeros(&md5::compute("abcdef1"), 5));
        assert!(!AdventCoin::digest_with_zeros(&md5::compute("pqrstuv1"), 5));
    }
}
