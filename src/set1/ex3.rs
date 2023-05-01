#![allow(dead_code)]

use std::collections::HashMap;

use super::ex1;

pub fn single_byte_xor(cyphertext: &Vec<u8>, key: u8) -> Vec<u8> {
    cyphertext.iter().map(|byte| byte ^ key).collect()
}

pub fn byte_freq_list(bytes: &Vec<u8>) -> Vec<(u8, u32)> {
    let mut freq = HashMap::new();

    for byte in bytes {
        match freq.get(byte) {
            Some(f) => freq.insert(byte, f + 1),
            None => freq.insert(byte, 1u32),
        };
    }

    let mut res = freq
        .into_iter()
        .map(|(byte, freq)| (*byte, freq))
        .collect::<Vec<(u8, u32)>>();

    res.sort_by_key(|(_, f)| *f);
    res.reverse();
    res
}

pub fn english_score(s: &String) -> Option<usize> {
    let freq_table = ['e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd', 'l', 'u'];
    let special_chars = [
        '+', '~', '{', '}', '[', ']', '*', '$', '^', '@', '/', '<', '>', '&',
    ];
    let mut score = 0;
    let mut halving = 0;

    let mut frequencies = HashMap::new();
    for c in s.chars() {
        if (c as u8) > 127 {
            return None;
        }

        match frequencies.get(&c) {
            Some(freq) => frequencies.insert(c, freq + 1),
            None => frequencies.insert(c, 1u32),
        };
    }

    let mut frequencies = frequencies.into_iter().collect::<Vec<_>>();
    frequencies.sort_by_key(|(_, f)| *f);
    frequencies.reverse();
    let chars = frequencies
        .into_iter()
        .map(|(c, _)| c.to_lowercase().next().unwrap())
        .filter(|c| *c != ' ')
        .collect::<Vec<_>>();

    for (i, c) in chars.iter().enumerate() {
        if special_chars.contains(&c) {
            halving += 1;
        }

        let table_pos = freq_table.iter().position(|x| x == c);
        if let None = table_pos {
            if i < 6 {
                halving += 1;
            }
            if i < 3 {
                halving += 1;
            }
            continue;
        }

        let distance = (table_pos.unwrap() as i32 - i as i32).abs() as u32;
        if distance > 12 {
            continue;
        }

        score += 2usize.pow(12 - distance);
    }
    // println!("{}", score);
    // println!("{}", halving);

    score /= 2usize.pow(halving);

    if score == 0 {
        return None;
    }

    Some(score)
}

pub fn main() {
    let encoded_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = ex1::hex_decode(encoded_string).unwrap();

    let mut messages = Vec::new();
    for (c, _) in byte_freq_list(&bytes) {
        let key = c ^ ('e' as u8);

        let decrypted_message = single_byte_xor(&bytes, key)
            .into_iter()
            .map(|byte| byte as char)
            .collect::<String>();

        let score = english_score(&decrypted_message);
        if let Some(score) = score {
            messages.push((decrypted_message, score));
        }
    }

    messages.sort_by_key(|(_, score)| *score);
    for (message, score) in messages {
        println!("M: {message}\tScore: {score}");
    }
}
