use std::collections::HashMap;
use std::path::PathBuf;

use jwalk::WalkDirGeneric;

pub type FileSize = u64;
pub type CustomData = ((), Option<FileSize>);
pub type ParallelWalkDir = WalkDirGeneric<CustomData>;

pub type SizeBasedFileIndex = HashMap<FileSize, Vec<PathBuf>>;

pub type Hash = String;
pub type HashBasedFileIndex = HashMap<Hash, (FileSize, Vec<PathBuf>)>;
