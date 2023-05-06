#![allow(dead_code)]

use std::error::Error;

pub fn hex_decode(s: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let chars = s.chars().filter(|c| c.is_digit(16)).collect::<String>();
    if chars.len() % 2 != 0 {
        return Err("Invalid string length".into());
    }

    let bytes = (0..chars.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&chars[i..i + 2], 16))
        .collect::<Result<Vec<u8>, _>>();
    Ok(bytes?)
}

pub fn hex_encode(bytes: &Vec<u8>) -> String {
    let mut s = String::new();
    for byte in bytes {
        let mut chunks = Vec::new();
        chunks.push((byte & 0b11110000) >> 4);
        chunks.push(byte & 0b00001111);

        for chunk in chunks {
            match chunk {
                0..=9 => s.push_str(chunk.to_string().as_str()),
                10 => s.push('a'),
                11 => s.push('b'),
                12 => s.push('c'),
                13 => s.push('d'),
                14 => s.push('e'),
                15 => s.push('f'),
                _ => unreachable!(),
            }
        }
    }
    s
}

pub fn b64_encode(bytes: &Vec<u8>) -> String {
    let mut s = (0..=bytes.len() / 3)
        .map(|i| {
            let mut bits_chunks = Vec::new();
            bits_chunks.push((bytes[i * 3] & 0b11111100) >> 2);

            if (i * 3 + 1) < bytes.len() {
                bits_chunks.push(
                    ((bytes[i * 3] & 0b00000011) << 4) | ((bytes[i * 3 + 1] & 0b11110000) >> 4),
                );

                if (i * 3 + 2) < bytes.len() {
                    bits_chunks.push(
                        ((bytes[i * 3 + 1] & 0b00001111) << 2)
                            | ((bytes[i * 3 + 2] & 0b11000000) >> 6),
                    );
                    bits_chunks.push(bytes[i * 3 + 2] & 0b00111111);
                } else {
                    bits_chunks.push((bytes[i * 3 + 1] & 0b00001111) << 2)
                }
            } else {
                bits_chunks.push((bytes[i * 3] & 0b00000011) << 4);
            }

            let mut s = String::new();
            for chunk in bits_chunks {
                match chunk {
                    0..=25 => s.push(('A' as u8 + chunk) as char),
                    26..=51 => s.push(('a' as u8 + (chunk - 26)) as char),
                    52..=61 => s.push_str((chunk - 52).to_string().as_str()),
                    62 => s.push('+'),
                    63 => s.push('/'),
                    _ => unreachable!(),
                }
            }
            s
        })
        .collect::<String>();

    if s.len() % 4 == 1 {
        s.push_str("===");
    } else if s.len() % 4 == 2 {
        s.push_str("==");
    } else if s.len() % 4 == 3 {
        s.push_str("=");
    }
    s
}
