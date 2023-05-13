use aes::cipher::{generic_array::GenericArray, BlockCipher, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;

pub fn ecb_encrypt(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(plaintext.len() % 16 == 0);
    assert!(key.len() == 16);

    let cypher = Aes128::new(GenericArray::from_slice(key));

    (0..plaintext.len())
        .step_by(16)
        .map(|i| {
            let mut block = GenericArray::clone_from_slice(&plaintext[i..i + 16]);
            cypher.encrypt_block(&mut block);
            block
        })
        .flatten()
        .collect()
}

pub fn ecb_decrypt(cyphertext: &[u8], key: &[u8]) -> Vec<u8> {
    assert_eq!(cyphertext.len() % 16, 0);
    assert_eq!(key.len(), 16);

    let cypher = Aes128::new(GenericArray::from_slice(key));

    let mut plaintext = (0..cyphertext.len())
        .step_by(16)
        .map(|i| {
            let mut block = GenericArray::clone_from_slice(&cyphertext[i..i + 16]);
            cypher.decrypt_block(&mut block);
            block
        })
        .flatten()
        .collect::<Vec<u8>>();

    let padding = *plaintext.last().unwrap() as usize;
    for _ in 0..padding {
        plaintext.pop();
    }
    plaintext
}

pub fn cbc_encrypt(plaintext: &[u8], iv: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(plaintext.len() % 16 == 0);
    assert!(key.len() == 16);
    assert!(iv.len() == 16);

    let cypher = Aes128::new(GenericArray::from_slice(key));

    let mut last_cyphertext = GenericArray::clone_from_slice(iv);

    (0..plaintext.len())
        .step_by(16)
        .map(|i| {
            let xored_block = plaintext[i..i + 16]
                .iter()
                .zip(last_cyphertext.iter())
                .map(|(b1, b2)| b1 ^ b2)
                .collect::<Vec<_>>();
            let mut block = GenericArray::clone_from_slice(xored_block.as_slice());
            cypher.encrypt_block(&mut block);
            last_cyphertext = block.clone();
            block
        })
        .flatten()
        .collect()
}

pub fn cbc_decrypt(cyphertext: &[u8], iv: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(cyphertext.len() % 16 == 0);
    assert!(key.len() == 16);
    assert!(iv.len() == 16);

    let cypher = Aes128::new(GenericArray::from_slice(key));

    (0..cyphertext.len())
        .step_by(16)
        .map(|i| {
            let mut block = GenericArray::clone_from_slice(&cyphertext[i..i + 16]);
            cypher.decrypt_block(&mut block);

            if i == 0 {
                block
                    .iter()
                    .zip(iv.iter())
                    .map(|(b1, b2)| b1 ^ b2)
                    .collect::<Vec<_>>()
            } else {
                block
                    .iter()
                    .zip(cyphertext[i - 16..i].iter())
                    .map(|(b1, b2)| b1 ^ b2)
                    .collect::<Vec<_>>()
            }
        })
        .flatten()
        .collect()
}
