# OTP File Encryption Tool

Encrypt your files securely with the uncrackable One-Time Pad (OTP) encryption method.

## Overview

This tool provides a command-line interface for encrypting and decrypting files using the OTP encryption technique. The One-Time Pad is renowned for its robust security, making it an ideal choice for protecting sensitive data.

## Features

- Encrypt files using OTP for maximum security.
- Decrypt files securely with the provided key.
- Generate random binary keys for each encryption.

## Usage

### Encryption

```bash
$ cargo run encrypt <input_file> [<output_file>] [<key_file>]
```

<input_file>: Path to the file you want to encrypt.
<output_file>: (Optional) Path to save the encrypted file. If not provided, a default name will be used.
<key_file>: (Optional) Path to the key file. If not provided, a new key will be generated.

## Decription

```bash
$ cargo run decrypt <input_file> <output_file> <key_file>
```

- `<input_file>`: Path to the file you want to decrypt.
- `<output_file>`: Path to save the decrypted file.
- `<key_file>`: Path to the key file used for encryption.
