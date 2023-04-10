// src/brute_decrypt.rs

use crate::decrypt::decrypt;
use crate::{char_to_number, find_inverse};

pub fn brute_force_decrypt(ciphertext: &str) -> Vec<String> {
    let mut results = Vec::new();

    for alpha in 1..26 {
        if find_inverse(alpha).is_some() {
            for beta in 0..26 {
                let decrypted_text = decrypt(ciphertext, alpha, beta);
                results.push(format!(
                    "Alpha: {}, Beta: {}, Text: {}",
                    alpha, beta, decrypted_text
                ));
            }
        }
    }

    results
}
