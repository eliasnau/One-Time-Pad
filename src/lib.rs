// src/lib.rs

use std::fs;
use std::io;

pub fn read_file_to_variable(file_path: &str) -> Result<Vec<u8>, io::Error> {
    let content = fs::read(file_path)?;
    Ok(content)
}

pub fn generate_random_binary(size: usize) -> Vec<u8> {
    (0..size).map(|_| rand::random::<u8>()).collect()
}

pub fn write_binary_file(file_path: &str, content: &[u8]) -> Result<(), io::Error> {
    fs::write(file_path, content)?;
    Ok(())
}

pub fn encrypt(original: &[u8], key: &[u8]) -> Vec<u8> {
    original
        .iter()
        .zip(key.iter())
        .map(|(&a, &b)| a ^ b)
        .collect()
}

pub fn decrypt(encrypted: &[u8], key: &[u8]) -> Vec<u8> {
    encrypt(encrypted, key) // Decrypting is the same as encrypting with the key
}

pub fn generate_or_read_key(key_file_path: &str, input_file_size: usize) -> Vec<u8> {
    match read_file_to_variable(key_file_path) {
        Ok(existing_key) => existing_key,
        Err(_) => {
            let new_key = generate_random_binary(input_file_size);
            if let Err(err) = write_binary_file(key_file_path, &new_key) {
                eprintln!("Error writing key file: {:?}", err);
            }
            new_key
        }
    }
}
