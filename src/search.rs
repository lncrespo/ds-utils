use std::collections::HashMap;
use std::fs;
use std::os::linux::fs::MetadataExt;
use std::path::Path;
use std::path::PathBuf;

use crate::{error, Config};

use indexmap::IndexMap;

pub fn search_target_directories(config: &Config) -> HashMap<&str, u64> {
    let mut error_stack: HashMap<&str, error::Error> = HashMap::new();
    let mut inodes: Vec<u64> = Vec::new();
    let mut results: HashMap<&str, u64> = HashMap::new();

    for directory in &config.directories {
        let directory_size: u64 = match get_fs_entry_size(Path::new(directory), &mut inodes) {
            Ok(v) => v,
            Err(e) => {
                error_stack.insert(directory, e);

                continue;
            }
        };

        if directory_size >= config.max_file_size {
            results.insert(directory, directory_size);
        }
    }

    if !error_stack.is_empty() {
        for (directory, error) in error_stack {
            eprintln!("{}: {}", directory, error);
        }
    }

    results
}

/// Returns the size of a file or directory (File System Entry) in bytes
///
/// If the given path is a directory, this function will recurse over its
/// contents, summing up the total size.
///
/// This function will ignore symbolic links, due to the potential risk of
/// corrupt symlink, as in straight broken symlinks (pointing to a non-existent location)
/// and as in symlinks pointing to the current directory (.), resulting in the program
/// continuing to recurse.
///
/// # Errors
///
/// * Given entry does not exist
/// * The process does not have permissions to access the entry
pub fn get_fs_entry_size(
    entry: &Path,
    hardlinked_inodes: &mut Vec<u64>,
) -> Result<u64, error::Error> {
    let mut size: u64 = 0;

    if entry.is_dir() {
        size += entry.metadata()?.len();

        for entry in fs::read_dir(entry)? {
            let path = entry?.path();

            if let Ok(_) = fs::read_link(&path) {
                size += path.symlink_metadata()?.len();

                continue;
            }

            if path.is_dir() {
                size += get_fs_entry_size(&path, hardlinked_inodes)?;
            } else {
                if path.metadata()?.st_nlink() > 1 {
                    let inode = path.metadata()?.st_ino();

                    if !hardlinked_inodes.contains(&inode) {
                        hardlinked_inodes.push(inode);

                        size += path.metadata()?.len();
                    }
                } else {
                    size += path.metadata()?.len();
                }
            }
        }
    } else {
        size = entry.metadata()?.len();
    };

    Ok(size)
}

pub fn get_largest_sub_entries(
    directory_path: &PathBuf,
    amount: u8,
) -> Result<IndexMap<PathBuf, u64>, error::Error> {
    let subdirectories: fs::ReadDir = fs::read_dir(directory_path)?;
    let mut inodes: Vec<u64> = Vec::new();
    let mut sub_entries: IndexMap<PathBuf, u64> = IndexMap::new();

    for subdirectory in subdirectories {
        if let Ok(v) = subdirectory {
            let path = v.path();
            let size = get_fs_entry_size(Path::new(&path), &mut inodes)?;

            sub_entries.insert(path, size);
        } else {
            break;
        };
    }

    sub_entries.sort_by(|_ak, av, _bk, bv| bv.cmp(av));

    if sub_entries.len() > amount.into() {
        sub_entries.split_off(amount.into());
    }

    Ok(sub_entries)
}
