// src/main.rs

use std::env;
use std::fs;

use otp;

fn print_usage() {
    println!("Usage: otp <encrypt/decrypt> <input_file> [<output_file>] [<key_file>]");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Error: Insufficient arguments.");
        print_usage();
        return;
    }

    let operation = &args[1];
    let input_file_path = &args[2];
    let output_file_path = if args.len() > 3 { Some(&args[3]) } else { None };
    let key_file_path = if args.len() > 4 { Some(&args[4]) } else { None };

    let input_file_size = fs::metadata(input_file_path).unwrap().len() as usize;

    let key = otp::generate_or_read_key(
        key_file_path.unwrap_or(&"key.bin".to_string()),
        input_file_size,
    );

    match operation.as_str() {
        "encrypt" => {
            let original_content = otp::read_file_to_variable(input_file_path).unwrap();
            let encrypted_content = otp::encrypt(&original_content, &key);

            match output_file_path {
                Some(ofp) => {
                    if let Err(err) = otp::write_binary_file(ofp, &encrypted_content) {
                        eprintln!("Error writing encrypted file: {:?}", err);
                    } else {
                        println!("Encryption successful! Encrypted file saved as: {}", ofp);
                    }
                }
                None => {
                    eprintln!("Error: Output file path not provided.");
                    print_usage();
                }
            }
        }
        "decrypt" => match key_file_path {
            Some(_kfp) => {
                let encrypted_content = otp::read_file_to_variable(input_file_path).unwrap();
                let decrypted_content = otp::decrypt(&encrypted_content, &key);

                match output_file_path {
                    Some(ofp) => {
                        if let Err(err) = otp::write_binary_file(ofp, &decrypted_content) {
                            eprintln!("Error writing decrypted file: {:?}", err);
                        } else {
                            println!("Decryption successful! Decrypted file saved as: {}", ofp);
                        }
                    }
                    None => {
                        eprintln!("Error: Output file path not provided.");
                        print_usage();
                    }
                }
            }
            None => {
                eprintln!("Error: Key file path not provided for decryption.");
                print_usage();
            }
        },
        _ => {
            println!("Error: Invalid operation. Use 'encrypt' or 'decrypt'.");
            print_usage();
        }
    }
}
