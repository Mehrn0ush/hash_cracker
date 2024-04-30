# Hash Cracker

A simple command-line tool written in Rust for cracking password hashes using various hashing algorithms.

## Features

- Supports cracking password hashes generated with SHA-1, SHA-256, MD5, and SHA-512 algorithms.
- Uses a wordlist to attempt to match the hashed passwords.
- Command-line interface for easy usage.

## Installation

1. Make sure you have Rust and Cargo installed. If not, follow the instructions [here](https://www.rust-lang.org/tools/install).
2. Clone this repository to your local machine:

    ```bash
    git clone https://github.com/your-username/hash-cracker.git
    ```

3. Navigate to the project directory:

    ```bash
    cd hash-cracker
    ```

4. Build the project:

    ```bash
    cargo build --release
    ```

5. Run the executable:

    ```bash
    ./target/release/hash_cracker <wordlist.txt> <hash> <algorithm>
    ```

    Replace `<wordlist.txt>` with the path to your wordlist file, `<hash>` with the hash you want to crack, and `<algorithm>` with the hashing algorithm used to generate the hash (supported algorithms: `sha1`, `sha256`, `md5`, `sha512`).

## Usage

```bash
./target/release/hash_cracker <wordlist.txt> <hash algorithm>
