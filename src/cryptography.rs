use std::collections::hash_map::{DefaultHasher};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use std::hash::{Hash, Hasher};
use std::process;
use crate::get_config::get_secret_str;

pub fn encrypt(psw: String, text: String) -> Vec<u8> {
    let key = Key::from_slice(psw.as_bytes());
    let cipher = Aes256Gcm::new(key);
    let secret = get_secret_str();
    let nonce = Nonce::from_slice(&secret.as_bytes());
    let ciphertext = match cipher.encrypt(nonce, text.as_ref()) {
        Ok(val) => val,
        Err(_err) => {
            println!("Encryption failed");
            process::exit(0);
        }
    };
    ciphertext
}

pub fn decrypt(psw: String, text: Vec<u8>) -> String {
    let key = Key::from_slice(psw.as_bytes());
    let cipher = Aes256Gcm::new(key);
    let secret = get_secret_str();
    let nonce = Nonce::from_slice(&secret.as_bytes());
    let plaintext = match cipher.decrypt(nonce, text.as_ref()) {
        Ok(val) => val,
        Err(_err) => {
            println!("Decryption failed");
            process::exit(0);
        }
    };
    let str_text = std::str::from_utf8(&plaintext).unwrap();
    str_text.to_string()
}

pub fn get_password_hash(psw: &String) -> String {
    let mut hasher = DefaultHasher::new();
    psw.hash(&mut hasher);
    let hash = hasher.finish().to_string();
    let mut big_hash = hash.clone() + &hash;
    while big_hash.len() > 32 {
        big_hash.pop();
    }
    big_hash
}