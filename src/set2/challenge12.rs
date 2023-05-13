use std::collections::HashMap;

use crate::{set1::challenge6::base_64_decode, set2::challenge10::ecb_encrypt};

use super::challenge9::pkcs7;
use rand::Rng;

const KEY: [u8; 16] = [
    0x7a, 0x92, 0xf6, 0xc6, 0x34, 0x1d, 0x9e, 0x7e, 0x5d, 0xc8, 0x4f, 0x56, 0xae, 0x0a, 0x1f, 0xcc,
];

pub fn ecb_unknown_key(plaintext: &[u8]) -> Vec<u8> {
    let suffix = base_64_decode(&String::from(
        "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\
aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\
YnkK",
    ))
    .unwrap();

    let mut plaintext = plaintext.to_vec();
    plaintext.extend_from_slice(suffix.as_slice());

    ecb_encrypt(pkcs7(plaintext.as_slice(), 16).as_slice(), &KEY)
}

pub fn discover_block_size() -> usize {
    let no_plaintext_size = ecb_unknown_key(vec![].as_slice()).len();

    let mut new_size = no_plaintext_size;
    let mut plaintext_size = 1;

    while new_size == no_plaintext_size {
        let plaintext = vec![0x00; plaintext_size];
        new_size = ecb_unknown_key(plaintext.as_slice()).len();
        plaintext_size += 1;
    }

    let old_size = new_size;
    let mut block_size = 0;

    while new_size == old_size {
        let plaintext = vec![0x00; plaintext_size];
        new_size = ecb_unknown_key(plaintext.as_slice()).len();
        block_size += 1;
        plaintext_size += 1;
    }

    block_size
}

pub fn decrypt_secret_string() -> Vec<u8> {
    // Discover how much padding there is
    let block_size = discover_block_size();
    let no_plaintext_size = ecb_unknown_key(vec![].as_slice()).len();
    let mut plaintext = vec![0u8];
    let mut with_plaintext_size = no_plaintext_size;
    let mut padding_bytes_n = 0usize;
    while with_plaintext_size == no_plaintext_size {
        padding_bytes_n += 1;
        with_plaintext_size = ecb_unknown_key(plaintext.as_slice()).len();
        plaintext.push(0u8);
    }

    let blocks_to_decrypt = no_plaintext_size / block_size;
    let mut base_plaintext = vec![0x00u8; no_plaintext_size - 1];
    let start_pos = (blocks_to_decrypt - 1) * block_size;

    let mut decrypted = Vec::new();

    while base_plaintext.len() > padding_bytes_n {
        // println!("Base plaintext length: {}", base_plaintext.len());
        let mut byte_map = HashMap::new();

        for byte in u8::MIN..=u8::MAX {
            let mut plaintext = base_plaintext.clone();
            plaintext.extend_from_slice(decrypted.as_slice());
            plaintext.push(byte);
            let cyphertext = ecb_unknown_key(plaintext.as_slice());
            byte_map.insert(cyphertext[start_pos..start_pos + block_size].to_vec(), byte);
        }

        let cyphertext = ecb_unknown_key(base_plaintext.as_slice());
        let correct_byte = byte_map
            .get(&cyphertext[start_pos..start_pos + block_size].to_vec())
            .unwrap_or_else(|| panic!("Unmatched blocks!"));

        decrypted.push(*correct_byte);
        base_plaintext.pop();
    }

    decrypted
}
