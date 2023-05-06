use aes::Aes128;
use aes::BlockDecrypt;
use aes::NewBlockCipher;
use cipher::generic_array::GenericArray;
use cipher::BlockCipher;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::challenge6::base_64_decode;

fn aes128_single_block_decrypt(ciphertext: &[u8], key: &Vec<u8>) -> Vec<u8> {
    assert_eq!(ciphertext.len(), 16);
    assert_eq!(key.len(), 16);

    let key = GenericArray::from_slice(key.as_slice());
    let mut ciphertext = GenericArray::clone_from_slice(ciphertext);
    let cipher = Aes128::new(&key);

    cipher.decrypt_block(&mut ciphertext);
    ciphertext.to_vec()
}

pub fn main() {
    let c_file = File::open("ciphertext7.txt").unwrap();
    let c_string = BufReader::new(c_file)
        .lines()
        .map(|s| s.unwrap())
        .collect::<String>();

    let ciphertext = base_64_decode(&c_string).unwrap();
    let key = "YELLOW SUBMARINE"
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<_>>();

    let mut plaintext = Vec::new();
    for i in (0..ciphertext.len()).step_by(16) {
        let block = &ciphertext[i..i + 16];
        let decrypted_block = aes128_single_block_decrypt(block, &key);
        for byte in decrypted_block {
            plaintext.push(byte);
        }
    }

    let plaintext = plaintext
        .into_iter()
        .map(|byte| byte as char)
        .collect::<String>();
    println!("{}", plaintext);
}
