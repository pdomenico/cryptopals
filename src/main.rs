mod set1;
mod set2;
use hex;
use set1::{challenge1, challenge3, challenge4, challenge5, challenge6, challenge7, challenge8};
use set2::{challenge11, challenge12};

fn main() {
    let decrypted = challenge12::decrypt_secret_string();
    println!(
        "{}",
        decrypted
            .iter()
            .map(|byte| *byte as char)
            .collect::<String>()
    );
}
