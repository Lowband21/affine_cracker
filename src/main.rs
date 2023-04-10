use std::env;
use std::fs;
use std::io;

mod brute_decrypt;
mod decrypt;
mod encrypt;

use affine_cracker::{char_to_number, find_inverse, number_to_char};
use brute_decrypt::brute_force_decrypt;
use decrypt::decrypt;
use encrypt::encrypt;

fn print_help() {
    println!("Usage: affine_cipher [OPTIONS] [FILENAME]");
    println!("Encrypts or decrypts a file using the affine cipher.");
    println!();
    println!("Options:");
    println!("  -e, --encrypt\t\tEncrypt the input text.");
    println!("  -d, --decrypt\t\tDecrypt the input text.");
    println!("  -a, --alpha [VALUE]\tSet the alpha value (default: 5).");
    println!("  -b, --beta [VALUE]\tSet the beta value (default: 8).");
    println!("  -h, --help\t\tDisplay this help message.");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut encrypt_mode = false;
    let mut decrypt_mode = false;
    let mut brute_decrypt_mode = false;
    let mut alpha = 5;
    let mut beta = 16;
    let mut filename = String::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-e" | "--encrypt" => {
                encrypt_mode = true;
                filename = "plaintext.txt".to_string();
                i += 1;
            }
            "-d" | "--decrypt" => {
                decrypt_mode = true;
                filename = "ciphertext.txt".to_string();
                i += 1;
            }
            "-bd" | "--brute_decrypt" => {
                brute_decrypt_mode = true;
                filename = "ciphertext.txt".to_string();
                i += 1;
            }
            "-a" | "--alpha" => {
                i += 1;
                alpha = args[i].parse::<i32>().unwrap_or(9);
                i += 1;
            }
            "-b" | "--beta" => {
                i += 1;
                beta = args[i].parse::<i32>().unwrap_or(2);
                i += 1;
            }
            "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            _ => {
                filename = args[i].clone();
                i += 1;
            }
        }
    }

    if !encrypt_mode && !decrypt_mode && !brute_decrypt_mode {
        println!("Error: Must specify either encryption (-e) or decryption (-d) mode.");
        print_help();
        return Ok(());
    }

    if encrypt_mode && decrypt_mode {
        println!("Error: Cannot specify both encryption (-e) and decryption (-d) modes.");
        print_help();
        return Ok(());
    }

    if filename.is_empty() {
        println!("Error: Must specify a filename or use --help for more information.");
        return Ok(());
    }

    let mut input: String = filename.clone();
    if filename.contains(".txt") {
        input = fs::read_to_string(&filename)?;
    }

    let output = if encrypt_mode {
        let input = input
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>();
        encrypt(&input, alpha, beta)
    } else if brute_decrypt_mode {
        let input = input
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_uppercase())
            .collect::<String>();
        let mut possible = brute_force_decrypt(&input);
        for i in &possible {
            println!("{}", i);
        }
        possible.pop().unwrap()
    } else {
        let input = input
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_uppercase())
            .collect::<String>();
        decrypt(&input, alpha, beta)
    };

    println!("{}", output);

    Ok(())
}
