use aes::Aes128;
use cipher::generic_array::GenericArray;
use cipher::BlockCipher;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::ex6::base_64_decode;

fn main() {
    let c_file = File::open("cyphertext7.txt").unwrap();
    let c_string = BufReader::new(c_file)
        .lines()
        .map(|s| s.unwrap())
        .collect::<String>();

    let cyphertext = base_64_decode(&c_string);
    let key = "YELLOW SUBMARINE"
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<_>>();
}
