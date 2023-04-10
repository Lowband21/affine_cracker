// src/lib.rs

// Converts a character to its corresponding number (A/a=0, B/b=1, ..., Z/z=25).
pub fn char_to_number(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        (c as u8 - b'a') as i32
    } else {
        (c as u8 - b'A') as i32
    }
}

// Converts a number to its corresponding uppercase or lowercase character.
pub fn number_to_char(number: i32, uppercase: bool) -> char {
    if uppercase {
        (number as u8 + b'A') as char
    } else {
        (number as u8 + b'a') as char
    }
}

// Finds the multiplicative inverse of a given number (mod 26).
pub fn find_inverse(alpha: i32) -> Option<i32> {
    for i in 1..26 {
        if (alpha * i) % 26 == 1 {
            return Some(i);
        }
    }
    None
}
