// src/encrypt.rs

use crate::{char_to_number, find_inverse, number_to_char};

pub fn encrypt(plaintext: &str, alpha: i32, beta: i32) -> String {
    let mut ciphertext = String::new();

    for c in plaintext.chars() {
        if c.is_ascii_alphabetic() {
            let is_uppercase = c.is_uppercase();
            let mut number = char_to_number(c);
            number = (alpha * number + beta) % 26;
            let encrypted_char = number_to_char(number, is_uppercase);
            ciphertext.push(encrypted_char);
        } else {
            ciphertext.push(c);
        }
    }

    ciphertext.to_uppercase()
}
