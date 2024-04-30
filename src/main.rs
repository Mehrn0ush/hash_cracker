use sha1::Digest;
use sha2::Sha256;

use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};
use hex;

const SHA1_HEX_STRING_LENGTH: usize = 40;
const SHA256_HEX_STRING_LENGTH: usize = 64;
const MD5_HEX_STRING_LENGTH: usize = 32;
const SHA512_HEX_STRING_LENGTH: usize = 128;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("hash_cracker: <wordlist.txt> <hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    let hash_algorithm = match hash_to_crack.len() {
        SHA1_HEX_STRING_LENGTH => HashAlgorithm::SHA1,
        SHA256_HEX_STRING_LENGTH => HashAlgorithm::SHA256,
        MD5_HEX_STRING_LENGTH => HashAlgorithm::MD5,
        SHA512_HEX_STRING_LENGTH => HashAlgorithm::SHA512,
        _ => return Err("Unsupported hash algorithm".into()),
    };

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();
        let hashed_password_hex = match hash_algorithm {
            HashAlgorithm::SHA1 => {
                let hash = sha1::Sha1::digest(common_password.as_bytes());
                hex::encode(hash)
            }
            HashAlgorithm::SHA256 => {
                let mut hasher = Sha256::new();
                hasher.update(common_password.as_bytes());
                let hash = hasher.finalize();
                hex::encode(hash)
            }
            HashAlgorithm::MD5 => {
                let hash = md5::compute(common_password);
                hex::encode(&hash[..]) // Convert `md5::Digest` to slice of bytes
            }
            HashAlgorithm::SHA512 => {
                let hash = sha2::Sha512::digest(common_password.as_bytes());
                hex::encode(hash)
            }
        };
        if hash_to_crack == hashed_password_hex {
            println!("Password found: {}", common_password);
            return Ok(());
        }
    }

    println!("Password not found in wordlist :(");
    Ok(())
}

enum HashAlgorithm {
    SHA1,
    SHA256,
    MD5,
    SHA512,
}
