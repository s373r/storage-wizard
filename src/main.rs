use std::collections::HashMap;
use std::env;

use jwalk::WalkDirGeneric;

fn main() {
    let root_path_for_scan = env::args().nth(1).unwrap_or("./".to_owned());
    let mut file_index = HashMap::<u64, Vec<String>>::new();

    for dir_entry_result in WalkDirGeneric::<((), u64)>::new(&root_path_for_scan)
        .skip_hidden(false)
        .process_read_dir(move |_, _, _, dir_entry_results| {
            dir_entry_results.iter_mut().for_each(|dir_entry_result| {
                if let Ok(dir_entry) = dir_entry_result {
                    if !dir_entry.file_type.is_dir() {
                        let file_size = dir_entry.metadata().map(|m| m.len()).unwrap_or_default();

                        dir_entry.client_state = file_size;
                    }
                }
            })
        })
    {
        match dir_entry_result {
            Ok(dir_entry) => {
                let file_size = dir_entry.client_state;
                let file_name = dir_entry.path().to_string_lossy().into_owned();
                let file_names = file_index.get_mut(&file_size);

                match file_names {
                    Some(v) => {
                        v.push(file_name);
                    }
                    None => {
                        let new_file_names = vec![file_name];
                        file_index.insert(file_size, new_file_names);
                    }
                };
            }
            Err(error) => {
                println!("Read dir_entry error: {}", error);
            }
        }
    }

    for (file_size, file_names) in file_index.into_iter().filter_map(|pair| {
        let (_, file_names) = &pair;

        if file_names.len() > 1 {
            Some(pair)
        } else {
            None
        }
    }) {
        println!("size {file_size}: {:?}", file_names);
    }
}
