#![allow(dead_code)]

use std::error::Error;

use super::challenge1;

pub fn fixed_xor(s1: &String, s2: &String) -> Result<String, Box<dyn Error>> {
    let b1 = challenge1::hex_decode(s1)?;
    let b2 = challenge1::hex_decode(s2)?;

    if b1.len() != b2.len() {
        return Err("Strings are not of equal lengths!".into());
    }

    Ok(challenge1::hex_encode(
        &b1.into_iter()
            .zip(b2.into_iter())
            .map(|(one, two)| one ^ two)
            .collect::<Vec<_>>(),
    ))
}
