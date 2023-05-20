use std::path::PathBuf;

use jwalk::rayon::prelude::*;
use jwalk::DirEntry;

use crate::common::hash_file_content;
use crate::types::*;

pub struct FileIndexBuilder<'a> {
    root_path: &'a str,
}

impl<'a> FileIndexBuilder<'a> {
    pub fn new(root_path: &'a str) -> Self {
        FileIndexBuilder { root_path }
    }

    pub fn build(self) -> HashBasedFileIndex {
        let walk_dir_iter = self.build_parallel_walk_dir_iter();
        let size_based_file_index = self.build_size_based_file_index(walk_dir_iter);

        self.build_hash_based_file_index(size_based_file_index)
    }

    fn build_parallel_walk_dir_iter(&self) -> ParallelWalkDir {
        ParallelWalkDir::new(self.root_path)
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
            })
    }

    fn build_size_based_file_index(&self, walk_dir_iter: ParallelWalkDir) -> SizeBasedFileIndex {
        // NOTE(DP): single-threaded index building
        let mut file_index = SizeBasedFileIndex::new();

        for dir_entry_result in walk_dir_iter {
            let Ok(dir_entry) = dir_entry_result else {
                unreachable!()
            };

            let Some(file_size) = dir_entry.client_state else {
                // NOTE(DP): skip dirs:
                //           file_size is None only for dirs
                continue;
            };

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

    fn build_hash_based_file_index(
        &self,
        size_based_file_index: SizeBasedFileIndex,
    ) -> HashBasedFileIndex {
        // NOTE(DP): parallel hashing & single-threaded index building
        let hashed_files: Vec<(PathBuf, Hash)> = size_based_file_index
            .into_iter()
            .par_bridge()
            .flat_map(|(_, file_paths)| file_paths)
            .map(|file_path| {
                let hash = hash_file_content(&file_path).unwrap();

                (file_path, hash)
            })
            .collect();

        let mut file_index = HashBasedFileIndex::new();

        for (file_path, hash) in hashed_files {
            let file_names = file_index.get_mut(&hash);

            match file_names {
                Some(v) => {
                    v.push(file_path);
                }
                None => {
                    let new_file_names = vec![file_path];
                    file_index.insert(hash, new_file_names);
                }
            };
        }

        file_index
    }
}
