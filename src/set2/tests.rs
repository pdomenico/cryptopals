#[cfg(test)]
mod challenge9_tests {
    use crate::set2::challenge9::*;

    #[test]
    fn first_test() {
        let result = pkcs7("YELLOW SUBMARINE".as_bytes(), 20);
        let expected = "YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes().to_vec();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pkcs7_no_padding_needed() {
        let input = vec![1, 2, 3, 4];
        let block_size = 4;
        let expected = vec![1, 2, 3, 4, 4, 4, 4, 4];
        let result = pkcs7(&input, block_size);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_pkcs7_partial_padding_needed() {
        let input = vec![1, 2, 3];
        let block_size = 4;
        let expected = vec![1, 2, 3, 1];
        let result = pkcs7(&input, block_size);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_pkcs7_full_padding_needed() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let block_size = 4;
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 4, 4, 4, 4];
        let result = pkcs7(&input, block_size);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_pkcs7_empty_input() {
        let input = vec![];
        let block_size = 4;
        let expected = vec![4, 4, 4, 4];
        let result = pkcs7(&input, block_size);
        assert_eq!(expected, result);
    }
}

#[cfg(test)]
mod challenge10_tests {
    use crate::set2::challenge10::*;
    use crate::set2::challenge9::pkcs7;

    use rand::rngs::OsRng;
    use rand::RngCore;

    fn generate_random_block() -> [u8; 16] {
        let mut block = [0u8; 16];
        OsRng.fill_bytes(&mut block);
        block
    }

    #[test]
    fn test_cbc_encrypt_decrypt() {
        let key = generate_random_block();
        let iv = generate_random_block();
        let plaintext = b"Hello, world! This is a test message for AES-128 CBC encryption.";

        // Pad the plaintext to make its length a multiple of 16
        let padded_plaintext = pkcs7(plaintext, 16);

        let encrypted = cbc_encrypt(&padded_plaintext, &iv, &key);
        let decrypted = cbc_decrypt(&encrypted, &iv, &key);

        assert_eq!(padded_plaintext, decrypted);
    }

    #[test]
    fn test_cbc_encrypt_decrypt_empty_input() {
        let key = generate_random_block();
        let iv = generate_random_block();
        let plaintext = b"";

        let padded_plaintext = pkcs7(plaintext, 16);

        let encrypted = cbc_encrypt(&padded_plaintext, &iv, &key);
        let decrypted = cbc_decrypt(&encrypted, &iv, &key);

        assert_eq!(padded_plaintext, decrypted);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_cbc_encrypt_invalid_plaintext_length() {
        let key = generate_random_block();
        let iv = generate_random_block();
        let plaintext = b"Invalid length";

        cbc_encrypt(plaintext, &iv, &key);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_cbc_decrypt_invalid_ciphertext_length() {
        let key = generate_random_block();
        let iv = generate_random_block();
        let cyphertext = b"Invalid length";

        cbc_decrypt(cyphertext, &iv, &key);
    }

    #[test]
    fn test_ecb_encryption_decryption() {
        let key = "supersecretkey!!"
            .chars()
            .map(|c| c as u8)
            .collect::<Vec<_>>(); // AES-128 requires a 16-byte
        let plaintext = "This is a test message."
            .chars()
            .map(|c| c as u8)
            .collect::<Vec<_>>();

        let ciphertext = ecb_encrypt(pkcs7(plaintext.as_slice(), 16).as_slice(), key.as_slice());
        let decrypted_text = ecb_decrypt(ciphertext.as_slice(), key.as_slice());

        assert_eq!(plaintext, decrypted_text);
    }
}

mod challenge13_tests {
    use crate::set2::challenge13::*;

    #[test]
    fn test_profile_for() {
        let email = "test@domain.com";
        let profile = profile_for(email);
        assert_eq!(profile, "email=test@domain.com&uid=10&role=user");
    }

    #[test]
    fn test_profile_for_with_metacharacters() {
        let email = "test@domain.com&role=admin";
        let profile = profile_for(email);
        // Metacharacters should be removed
        assert_eq!(profile, "email=test@domain.comroleadmin&uid=10&role=user");
    }

    #[test]
    fn test_parse_from_str_correctly_formatted() {
        let encoded_profile = "email=foo@bar.com&uid=3&role=user";
        let profile = Profile::parse_from_str(encoded_profile).unwrap();

        assert_eq!(profile.email, "foo@bar.com");
        assert_eq!(profile.uid, 3);
        assert_eq!(profile.role, "user");
    }

    #[test]
    fn test_parse_from_str_missing_email() {
        let encoded_profile = "uid=3&role=user";

        match Profile::parse_from_str(encoded_profile) {
            Ok(_) => panic!("Expected an error, but got Ok(_)"),
            Err(e) => assert_eq!(e, "Missing email"),
        }
    }

    #[test]
    fn test_parse_from_str_invalid_uid() {
        let encoded_profile = "email=foo@bar.com&uid=abc&role=user";

        match Profile::parse_from_str(encoded_profile) {
            Ok(_) => panic!("Expected an error, but got Ok(_)"),
            Err(e) => assert_eq!(e, "Invalid uuid"),
        }
    }

    #[test]
    fn test_parse_from_str_missing_role() {
        let encoded_profile = "email=foo@bar.com&uid=3";

        match Profile::parse_from_str(encoded_profile) {
            Ok(_) => panic!("Expected an error, but got Ok(_)"),
            Err(e) => assert_eq!(e, "Missing role"),
        }
    }
}
