use std::collections::HashMap;

use super::{
    challenge10::{ecb_decrypt, ecb_encrypt},
    challenge9::pkcs7,
};

const KEY: [u8; 16] = [
    0x7a, 0x92, 0xf6, 0xc6, 0x34, 0x1d, 0x9e, 0x7e, 0x5d, 0xc8, 0x4f, 0x56, 0xae, 0x0a, 0x1f, 0xcc,
];

#[derive(Debug)]
pub struct Profile {
    pub email: String,
    pub uid: u32,
    pub role: String,
}

impl Profile {
    pub fn parse_from_str(encoded_profile: &str) -> Result<Profile, &'static str> {
        let credentials: HashMap<_, _> = encoded_profile
            .split('&')
            .filter_map(|part| {
                let mut split = part.splitn(2, '=');
                Some((split.next()?, split.next()?))
            })
            .collect();

        Ok(Profile {
            email: String::from(*credentials.get("email").ok_or("Missing email")?),
            uid: credentials
                .get("uid")
                .ok_or("Mising uid")?
                .parse()
                .map_err(|_| "Invalid uuid")?,
            role: String::from(*credentials.get("role").ok_or("Missing role")?),
        })
    }
}

pub fn profile_for(email: &str) -> String {
    let email: String = email
        .chars()
        .filter_map(|c| {
            if vec!['&', '='].contains(&c) {
                None
            } else {
                Some(c)
            }
        })
        .collect();

    String::from(format!("email={email}&uid=10&role=user"))
}

pub fn ecb_encrypt_static_key(plaintext: &[u8]) -> Vec<u8> {
    ecb_encrypt(pkcs7(plaintext, 16).as_slice(), &KEY)
}

pub fn ecb_decrypt_static_key(cyphertext: &[u8]) -> Vec<u8> {
    ecb_decrypt(cyphertext, &KEY)
}

pub fn get_admin_user() {
    let email_13_bytes = "AAAAAAAAAAAAA";
    let plaintext: Vec<u8> = profile_for(email_13_bytes)
        .chars()
        .map(|c| c as u8)
        .collect();
    let first_cyphertext = ecb_encrypt_static_key(plaintext.as_slice());

    let mut admin_mail = String::from("AAAAAAAAAAadmin");
    admin_mail.push_str(
        vec![11u8; 11]
            .into_iter()
            .map(|byte| byte as char)
            .collect::<String>()
            .as_str(),
    );
    let plaintext: Vec<u8> = profile_for(admin_mail.as_str())
        .chars()
        .map(|c| c as u8)
        .collect();

    let second_cyphertext = ecb_encrypt_static_key(plaintext.as_slice());

    let mut final_cyphertext = first_cyphertext.clone();
    for _ in 0..16 {
        final_cyphertext.pop();
    }
    for i in 16..32 {
        final_cyphertext.push(second_cyphertext[i]);
    }

    let admin_encoding = ecb_decrypt_static_key(final_cyphertext.as_slice())
        .iter()
        .map(|byte| *byte as char)
        .collect::<String>();
    let admin_user = Profile::parse_from_str(admin_encoding.as_str());
    println!("Admin: {:?}", admin_user);
}
