mod common;
mod file_index;
mod types;

use std::env;

use crate::file_index::*;

fn main() {
    let root_path_for_scan = env::args().nth(1).unwrap_or("./".to_owned());

    let walk_dir_iter = build_parallel_walk_dir_iter(root_path_for_scan.as_str());
    let size_based_file_index = build_size_based_file_index(walk_dir_iter);
    let hash_based_file_index = build_hash_based_file_index(size_based_file_index);

    for (hash, file_names) in hash_based_file_index
        .into_iter()
        .filter(|(_, file_names)| file_names.len() > 1)
    {
        println!("hash {hash}: {:?}", file_names);
    }
}
