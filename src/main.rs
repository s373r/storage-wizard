mod common;
mod file_index;
mod types;

use std::env;

use crate::file_index::FileIndexBuilder;

fn main() {
    let root_path_for_scan = env::args().nth(1).unwrap_or("./".to_owned());
    let file_index = FileIndexBuilder::new(root_path_for_scan.as_str()).build();

    for (hash, file_names) in file_index
        .into_iter()
        .filter(|(_, file_names)| file_names.len() > 1)
    {
        println!("hash {hash}: {:?}", file_names);
    }
}
