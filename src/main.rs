use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

use jwalk::{DirEntry, WalkDirGeneric};

type FileSize = u64;
type CustomData = ((), Option<FileSize>);
type ParallelWalkDir = WalkDirGeneric<CustomData>;

fn build_size_based_file_index(walk_dir_iter: ParallelWalkDir) -> HashMap<FileSize, Vec<PathBuf>> {
    let mut file_index = HashMap::<FileSize, Vec<PathBuf>>::new();

    for dir_entry_result in walk_dir_iter {
        let Ok(dir_entry) = dir_entry_result else {
            unreachable!()
        };

        let Some(file_size) = dir_entry.client_state else {
            // NOTE(DP): skip dirs:
            //           file_size is None only for dirs
            continue;
        };

        // let file_name = dir_entry.path().to_string_lossy().into_owned();
        let file_names = file_index.get_mut(&file_size);

        match file_names {
            Some(v) => {
                v.push(dir_entry.path());
            }
            None => {
                let new_file_names = vec![dir_entry.path()];
                file_index.insert(file_size, new_file_names);
            }
        };
    }

    file_index
}

fn main() {
    let root_path_for_scan = env::args().nth(1).unwrap_or("./".to_owned());

    // NOTE(DP): iterator preparation
    let walk_dir_iter = ParallelWalkDir::new(&root_path_for_scan)
        .skip_hidden(false)
        .process_read_dir(|_, _, _, dir_entry_results| {
            // NOTE(DP): yield only files & dirs and entities w/o errors
            dir_entry_results.retain(|dir_entry_result| match dir_entry_result {
                Ok(DirEntry { file_type, .. }) => file_type.is_file() || file_type.is_dir(),
                Err(error) => {
                    eprintln!("Read dir_entry error: {}", error);

                    false
                }
            });

            dir_entry_results.iter_mut().for_each(|dir_entry_result| {
                let Ok(dir_entry) = dir_entry_result else {
                        // NOTE(DP): covered by the previous retain() call
                        return;
                    };

                if !dir_entry.file_type.is_file() {
                    return;
                }

                let file_size = dir_entry.metadata().map(|m| m.len()).unwrap();

                dir_entry.client_state = Some(file_size);
            });
        });

    let file_index = build_size_based_file_index(walk_dir_iter);

    for (file_size, file_names) in file_index
        .into_iter()
        .filter(|(_, file_names)| file_names.len() > 2)
    {
        println!("size {file_size}: {:?}", file_names);
    }
}
