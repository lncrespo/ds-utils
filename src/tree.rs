use std::collections::HashMap;
use std::path::PathBuf;

use crate::config::Config;

use crate::error;
use crate::search;

use search::get_largest_sub_entries;

static TREE_ENTRY_SIGN: &str = "├─";
static INDENTATION_UNIT: &str = "    ";

pub fn print_tree(config: &Config, directories: &HashMap<&str, u64>) -> Result<(), error::Error> {
    for (path, size) in directories {
        println!("\n{} - {}\n░░░░░░░░░░░░░░░░", path, size);

        walk_entries(
            &PathBuf::from(path),
            config.tree_length,
            0,
            config.tree_depth,
        )?;
    }

    Ok(())
}

fn walk_entries(
    directory: &PathBuf,
    max_length: u8,
    current_depth: u8,
    max_depth: u8,
) -> Result<(), error::Error> {
    if current_depth >= max_depth && max_depth != 0 {
        return Ok(());
    }

    if directory.is_dir() {
        let entries = get_largest_sub_entries(directory, max_length)?;

        for (path, size) in entries {
            display_tree_node(&path, size, current_depth);

            if path.is_dir() {
                let current_depth = current_depth + 1;

                walk_entries(&path, max_length, current_depth, max_depth)?;
            }
        }
    }

    Ok(())
}

fn display_tree_node(entry: &PathBuf, size: u64, depth: u8) {
    let indentation: String = INDENTATION_UNIT.repeat(depth.into());

    println!(
        "{size: <15}░ {indentation}{tree_sign} {entry_name}",
        size = size,
        indentation = indentation,
        tree_sign = TREE_ENTRY_SIGN,
        entry_name = entry.display()
    );
}
