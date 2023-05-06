use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::{challenge1, challenge3};

pub fn repeating_score(ciphertext: &Vec<u8>, length: usize) -> u32 {
    let mut unique_patterns = HashMap::new();

    for i in (0..ciphertext.len()).step_by(length) {
        let pattern = &ciphertext[i..i + length];
        match unique_patterns.entry(pattern) {
            Entry::Vacant(entry) => {
                entry.insert(0);
            }
            Entry::Occupied(mut entry) => {
                let value = entry.get_mut();
                *value += 1;
            }
        }
    }

    unique_patterns.into_iter().map(|(_, count)| count).sum()
}

pub fn main() {
    let c_file = File::open("ciphertext8.txt").unwrap();
    let cyphertexts = BufReader::new(c_file)
        .lines()
        .map(|l| challenge1::hex_decode(l.unwrap().as_str()).unwrap())
        .collect::<Vec<_>>();

    let cyphertexts_scores = cyphertexts
        .iter()
        .map(|cyphertext| repeating_score(cyphertext, 16));

    let ecb_cyphertext = cyphertexts_scores
        .enumerate()
        .max_by_key(|(_, score)| *score)
        .unwrap();

    println!(
        "The cypertext encrypted with ECB is the number {}, with repeating score {}",
        ecb_cyphertext.0, ecb_cyphertext.1
    );
}
