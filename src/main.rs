mod common;
mod file_index;
mod types;

use std::env;

use colored::*;

use crate::file_index::FileIndexBuilder;

fn main() {
    let root_path_for_scan = env::args().nth(1).unwrap_or("./".to_owned());
    let file_index = FileIndexBuilder::new(root_path_for_scan.as_str()).build();

    if file_index.is_empty() {
        println!("No file duplicates");
        return;
    }

    file_index
        .into_iter()
        .filter(|(_, (_, file_names))| file_names.len() > 1)
        .for_each(|(hash, (file_size, file_names))| {
            println!(
                "{} {} {} {} {}",
                "- Hash".dimmed(),
                hash.green(),
                ", ".dimmed(),
                "size ".dimmed(),
                file_size.to_string().cyan()
            );

            let duplicate_count = file_names.len();

            for (index, file_name) in file_names.iter().enumerate() {
                let duplicate_number = index + 1;
                let file_name = file_name.to_string_lossy();

                println!(
                    "  {} {}",
                    format!("[{duplicate_number}/{duplicate_count}]")
                        .dimmed()
                        .bold(),
                    file_name
                );
            }

            println!();
        });
}
