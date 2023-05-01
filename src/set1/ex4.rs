#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};

use super::{ex1, ex3};

pub fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

pub fn bytes_to_text(bytes: &Vec<u8>) -> String {
    bytes.iter().map(|byte| *byte as char).collect()
}

pub fn try_decrypt() {
    let lines = read_file("src/set1/cyphertext.txt");
    let mut messages = Vec::new();

    for bytes in lines
        .iter()
        .map(|line| ex1::hex_decode(line.as_str()).unwrap())
    {
        let freq_table = ex3::byte_freq_list(&bytes);
        for (byte, _) in freq_table {
            let decrypted = ex3::single_byte_xor(&bytes, byte ^ ('e' as u8));
            let text = bytes_to_text(&decrypted);
            let score = ex3::english_score(&text);
            if let Some(score) = score {
                messages.push((text, score));
            }
        }
    }

    messages.sort_by_key(|(_, score)| *score);
    messages.reverse();
    for (m, _) in messages {
        println!("{}", m);
    }
}
