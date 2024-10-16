use md5::Digest;

pub struct AdventCoin {}

impl AdventCoin {
    pub fn mine(secret_key: &str) -> u32 {
        let mut suffix = 1;

        loop {
            let number = format!("{}{}", secret_key, suffix);
            let digest = md5::compute(&number);

            if Self::digest_with_zeros(&digest) {
                break;
            }
            suffix += 1;
        }

        suffix
    }

    // Check if hash digest contains leading 5 zeros
    fn digest_with_zeros(digest: &Digest) -> bool {
        // Check if the first 5 hexadecimal digits (20 bits) are zero
        digest[0] == 0 && digest[1] == 0 && digest[2] & 0xF0 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Long running test
    fn test_mine() {
        assert_eq!(AdventCoin::mine("abcdef"), 609043);
        assert_eq!(AdventCoin::mine("pqrstuv"), 1048970);
    }

    #[test]
    fn test_digest_with_zeros() {
        assert!(AdventCoin::digest_with_zeros(&md5::compute("abcdef609043")));
        assert!(AdventCoin::digest_with_zeros(&md5::compute(
            "pqrstuv1048970"
        )));
    }

    #[test]
    fn test_digest_with_non_zeros() {
        assert!(!AdventCoin::digest_with_zeros(&md5::compute("abcdef1")));
        assert!(!AdventCoin::digest_with_zeros(&md5::compute("pqrstuv1")));
    }
}
