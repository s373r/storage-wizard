mod common;
mod file_index;
mod types;

use std::env;

use crate::file_index::FileIndexBuilder;

fn main() {
    let root_path_for_scan = env::args().nth(1).unwrap_or("./".to_owned());
    let file_index = FileIndexBuilder::new(root_path_for_scan.as_str()).build();

    if file_index.is_empty() {
        println!("No file duplicates");
        return;
    }

    println!("Found {} duplicate groups:", file_index.len());

    file_index
        .into_iter()
        .filter(|(_, file_names)| file_names.len() > 1)
        .for_each(|(hash, file_names)| {
            println!("Hash {hash}");

            for file_name in file_names {
                let file_name = file_name.to_string_lossy();

                println!("{file_name}");
            }

            println!();
        });
}
