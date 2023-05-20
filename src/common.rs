use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use sha2::{Digest, Sha256};

pub fn hash_file_content(path: &PathBuf) -> io::Result<String> {
    let file = File::open(path)?;
    let mut reader = io::BufReader::new(file);

    let digest = {
        let mut hasher = Sha256::new();
        // NOTE(DP): https://eklitzke.org/efficient-file-copying-on-linux
        let mut buffer = [0; 128 * 1024];

        loop {
            let count = reader.read(&mut buffer)?;

            if count == 0 {
                break;
            }

            hasher.update(&buffer[..count]);
        }

        hasher.finalize()
    };

    Ok(format!("{:x}", digest))
}
