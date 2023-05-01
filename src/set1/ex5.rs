#![allow(dead_code)]
use super::ex1;

fn repeating_xor(message: &String, key: &Vec<u8>) -> Vec<u8> {
    let mut cyphertext = Vec::new();
    for (i, c) in message.chars().enumerate() {
        cyphertext.push((c as u8) ^ key[i % key.len()]);
    }
    cyphertext
}

pub fn main() {
    let message = String::from(
        "Burning 'em, if you ain't quick and nimble \
        I go crazy when I hear a cymbal",
    );

    let key = "ICE".chars().map(|c| c as u8).collect();
    let cyphertext = repeating_xor(&message, &key);
    println!("{}", ex1::hex_encode(&cyphertext));
}
