#![allow(dead_code)]

use std::{collections::HashMap, error::Error, fs::File, io::BufRead, io::BufReader};

use crate::set1::{challenge3, challenge5};

use super::challenge3::english_score;

pub fn count_ones(byte: u8) -> u32 {
    let mut mask = 0b00000001u8;
    let mut count = 0;

    for _ in 0..8 {
        if (byte & mask) != 0 {
            count += 1;
        }
        mask <<= 1;
    }
    count
}

pub fn hamming_distance(bytes1: &Vec<u8>, bytes2: &Vec<u8>) -> Option<u32> {
    if bytes1.len() != bytes2.len() {
        return None;
    }

    Some(
        bytes1
            .iter()
            .zip(bytes2.iter())
            .map(|(b1, b2)| count_ones(b1 ^ b2))
            .sum(),
    )
}

pub fn find_keylength(cyphertext: &Vec<u8>) -> Vec<usize> {
    let mut distances = HashMap::new();

    for keylength in 2..=40 {
        let first_pair = (
            &cyphertext[0..keylength],
            &cyphertext[keylength..2 * keylength],
        );
        let second_pair = (
            &cyphertext[2 * keylength..3 * keylength],
            &cyphertext[3 * keylength..4 * keylength],
        );

        let first_distance = hamming_distance(&first_pair.0.to_vec(), &first_pair.1.to_vec())
            .unwrap() as f64
            / keylength as f64;
        let second_distance = hamming_distance(&second_pair.0.to_vec(), &second_pair.1.to_vec())
            .unwrap() as f64
            / keylength as f64;

        let average_distance = (first_distance + second_distance) as f64 / 2.0;

        distances.insert(keylength, average_distance);
    }

    let mut distances = distances.into_iter().collect::<Vec<_>>();
    distances.sort_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap());
    // println!("Distances {:?}", distances);
    distances
        .into_iter()
        .map(|(keylength, _)| keylength)
        .collect()
}

pub fn base_64_char_to_bits(c: char) -> Option<u8> {
    match c {
        'A'..='Z' => Some((c as u8) - 65),
        'a'..='z' => Some((c as u8) - 97 + 26),
        '0'..='9' => Some((c as u8) - 48 + 52),
        '+' => Some(62),
        '/' => Some(63),
        '=' => None,
        _ => unreachable!(),
    }
}

pub fn base_64_decode(s: &String) -> Result<Vec<u8>, Box<dyn Error>> {
    if s.len() % 4 != 0 {
        return Err("Invalid string length!".into());
    }
    let s = s.chars().collect::<Vec<char>>();

    let mut bytes = Vec::new();

    for i in 0..(s.len() / 4) {
        let chunks = &s[i * 4..i * 4 + 4]
            .iter()
            .map(|c| base_64_char_to_bits(*c))
            .collect::<Vec<_>>();

        if let Some(chunk2) = chunks[1] {
            bytes.push((chunks[0].unwrap() << 2) | ((chunk2 & 0b00110000) >> 4));

            if let Some(chunk3) = chunks[2] {
                bytes.push(((chunk2 & 0b00001111) << 4) | ((chunk3 & 0b00111100) >> 2));

                if let Some(chunk4) = chunks[3] {
                    bytes.push(((chunk3 & 0b00000011) << 6) | chunk4);
                }
            }
        } else {
            return Err("Invalid string length!".into());
        }
    }

    Ok(bytes)
}

pub fn split_c_blocks(cyphertext_bytes: &Vec<u8>, keylength: usize) -> Vec<Vec<u8>> {
    if cyphertext_bytes.is_empty() {
        return vec![];
    }

    (0..((cyphertext_bytes.len() - 1) / keylength as usize) + 1)
        .into_iter()
        .map(|i| {
            (i * keylength..(i + 1) * keylength)
                .into_iter()
                .filter_map(|j| cyphertext_bytes.get(j))
                .map(|&x| x)
                .collect()
        })
        .collect()
}

pub fn transpose_blocks(blocks: &Vec<Vec<u8>>, keylength: usize) -> Vec<Vec<u8>> {
    (0..keylength)
        .into_iter()
        .map(|i| {
            blocks
                .iter()
                .filter_map(|block| block.get(i as usize))
                .map(|x| *x)
                .collect()
        })
        .collect()
}

pub fn repeating_key_xor(cyphertext: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    cyphertext
        .iter()
        .enumerate()
        .map(|(i, byte)| byte ^ key[i % key.len()])
        .collect()
}

pub fn main() {
    let cyphertext_file = File::open("cyphertext6.txt").unwrap();
    let cyphertext = BufReader::new(cyphertext_file)
        .lines()
        .map(|s| s.unwrap())
        .collect::<String>();

    let cyphertext_bytes = base_64_decode(&cyphertext).unwrap();

    let keylengths = find_keylength(&cyphertext_bytes);
    let mut plaintexts = Vec::new();

    for keylength in keylengths {
        let blocks = split_c_blocks(&cyphertext_bytes, keylength);
        let transposed_blocks = transpose_blocks(&blocks, keylength);

        let mut key = vec![0u8; keylength];
        for (i, block) in transposed_blocks.iter().enumerate() {
            let mut char_scores = (0u8..=255u8)
                .map(|key| {
                    (
                        key,
                        challenge3::english_score(
                            &block
                                .iter()
                                .map(|block_byte| (block_byte ^ key) as char)
                                .collect::<String>(),
                        ),
                    )
                })
                .filter(|(_, score)| score.is_some())
                .map(|(char, score)| (char, score.unwrap()))
                .collect::<Vec<_>>();
            char_scores.sort_by_key(|(_, score)| *score);
            char_scores.reverse();
            if !char_scores.is_empty() {
                key[i] = char_scores[0].0;
            }
        }
        plaintexts.push(
            repeating_key_xor(&cyphertext_bytes, &key)
                .iter()
                .map(|byte| *byte as char)
                .collect::<String>(),
        );
    }

    plaintexts.sort_by_key(|plaintext| english_score(plaintext));
    plaintexts.reverse();
    println!("{}", plaintexts[0]);
}
