use hex::encode;
use rand::rngs::OsRng;
use rand::{thread_rng, Rng, RngCore};

use crate::set2::challenge10::{cbc_encrypt, ecb_encrypt};
use crate::set2::challenge9::pkcs7;

#[derive(PartialEq, Eq)]
pub enum EncryptionMode {
    CBC,
    ECB,
}

pub fn random_block() -> [u8; 16] {
    let mut key = [0u8; 16];
    OsRng.fill_bytes(&mut key);
    key
}

pub fn encryption_oracle(input: &[u8]) -> (Vec<u8>, EncryptionMode) {
    let mut rng = rand::thread_rng();
    // let encryption_mode = match rng.gen_bool(0.5) {
    //     true => EncryptionMode::CBC,
    //     false => EncryptionMode::ECB,
    // };
    let encryption_mode = EncryptionMode::ECB;

    let additional_bytes_n: usize = rng.gen_range(5..=10);
    let additional_bytes = (0..additional_bytes_n)
        .map(|_| rng.gen())
        .collect::<Vec<u8>>();

    let plaintext = additional_bytes
        .iter()
        .chain(input.iter())
        .chain(additional_bytes.iter())
        .copied()
        .collect::<Vec<_>>();

    let padded_plaintext = pkcs7(plaintext.as_slice(), 16);

    print!("Padded plaintext: ");
    for i in (0..padded_plaintext.len()).step_by(16) {
        print!("{} ", encode(&padded_plaintext[i..i + 16]));
    }
    println!();

    match encryption_mode {
        EncryptionMode::ECB => (
            ecb_encrypt(padded_plaintext.as_slice(), &random_block()),
            EncryptionMode::ECB,
        ),
        EncryptionMode::CBC => (
            cbc_encrypt(
                padded_plaintext.as_slice(),
                &random_block(),
                &random_block(),
            ),
            EncryptionMode::CBC,
        ),
    }
}

pub fn mode_detector() -> f64 {
    let tests_n = 1;
    let mut correct = 0;

    let mut rng = thread_rng();

    for _ in 0..tests_n {
        let plaintext = (0..32).map(|_| rng.gen()).collect::<Vec<u8>>();
        let (cyphertext, real_mode) = encryption_oracle(plaintext.as_slice());

        print!("Cyphertext: ");
        for i in (0..cyphertext.len()).step_by(16) {
            print!("{} ", encode(&cyphertext[i..i + 16]));
        }
        println!();

        let mut guess = EncryptionMode::CBC;

        // TODO

        if guess == real_mode {
            correct += 1;
        }
    }

    correct as f64 / tests_n as f64
}

pub fn main() {
    println!("Predictor performance: {}", mode_detector());
}
