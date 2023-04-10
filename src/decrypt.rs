// src/decrypt.rs

use crate::{char_to_number, find_inverse, number_to_char};

pub fn decrypt(ciphertext: &str, alpha: i32, beta: i32) -> String {
    if alpha == 2 {
        println!("Invalid alpha, falling back to caesar");
        return decrypt_caesar(ciphertext, beta);
    }
    let mut plaintext = String::new();
    let alpha_inv = match find_inverse(alpha) {
        Some(a_inv) => a_inv,
        None => panic!(
            "Invalid value for alpha: {}. Multiplicative inverse not found.",
            alpha
        ),
    };

    for c in ciphertext.chars() {
        if c.is_ascii_alphabetic() {
            let is_uppercase = c.is_uppercase();
            let mut number = char_to_number(c);
            number = (alpha_inv * (number - beta + 26)) % 26;
            let decrypted_char = number_to_char(number, is_uppercase);
            plaintext.push(decrypted_char);
        } else {
            plaintext.push(c);
        }
    }

    plaintext
}

fn decrypt_caesar(ciphertext: &str, shift: i32) -> String {
    let mut plaintext = String::new();

    for c in ciphertext.chars() {
        if c.is_ascii_alphabetic() {
            let is_uppercase = c.is_uppercase();
            let mut number = char_to_number(c);
            number = (number - shift + 26) % 26;
            let decrypted_char = number_to_char(number, is_uppercase);
            plaintext.push(decrypted_char);
        } else {
            plaintext.push(c);
        }
    }

    plaintext
}
