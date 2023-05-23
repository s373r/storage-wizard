mod cli;
mod common;
mod file_index;
mod types;

use std::error::Error;
use std::path::PathBuf;

use clap::Parser;
use colored::*;
use indicatif::HumanBytes;

use crate::cli::Cli;
use crate::common::hash_file_content;
use crate::file_index::{FileGroup, FileIndexBuilder, FileIndexStorage};

const FILE_INDEX_STORAGE_PATH: &str = "storage-wizard.index.json";

fn main() -> Result<(), Box<dyn Error>> {
    let Cli {
        path: root_path_for_scan,
        no_file_index,
        search,
        ignore_file_index,
    } = Cli::parse();
    let file_index = if !search.is_empty() && !ignore_file_index {
        FileIndexBuilder::from_file(FILE_INDEX_STORAGE_PATH)?
    } else {
        FileIndexBuilder::new(root_path_for_scan.as_str()).build()
    };

    if file_index.is_empty() {
        eprintln!("No file duplicates");

        return Ok(());
    }

    let mut file_index_storage = FileIndexStorage::new();

    if !search.is_empty() {
        for search_file_path in search {
            println!("Search {}", search_file_path);

            let search_file_path = PathBuf::from(search_file_path);
            let hash = hash_file_content(&search_file_path)?;

            match file_index.get(&hash) {
                Some((_, file_names)) => {
                    for file_name in file_names {
                        if *file_name == search_file_path {
                            continue;
                        }

                        println!("{}", format!("+ {}", file_name.display()).green());
                    }
                }
                None => println!("{}", "-".red()),
            };

            println!()
        }
    } else {
        file_index
            .into_iter()
            .filter(|(_, (_, file_names))| file_names.len() > 1)
            .for_each(|(hash, (file_size, file_names))| {
                println!(
                    "{} {} {} {}",
                    "- Hash".dimmed(),
                    hash.green(),
                    "size ".dimmed(),
                    HumanBytes(file_size).to_string().cyan()
                );

                let duplicate_count = file_names.len();
                let escaped_file_names: Vec<_> = file_names
                    .iter()
                    .map(|file_name| file_name.to_string_lossy())
                    .collect();

                for (index, file_name) in escaped_file_names.iter().enumerate() {
                    let duplicate_number = index + 1;

                    println!(
                        "  {} {}",
                        format!("[{duplicate_number}/{duplicate_count}]")
                            .dimmed()
                            .bold(),
                        file_name
                    );
                }

                if !no_file_index {
                    let files = escaped_file_names
                        .into_iter()
                        .map(|name| name.into_owned())
                        .collect();
                    let file_group = FileGroup {
                        hash,
                        files,
                        size: file_size,
                    };

                    file_index_storage.groups.push(file_group);
                }

                println!();
            });

        if !no_file_index {
            std::fs::write(
                FILE_INDEX_STORAGE_PATH,
                serde_json::to_string_pretty(&file_index_storage).unwrap(),
            )?;
        }
    }

    Ok(())
}
