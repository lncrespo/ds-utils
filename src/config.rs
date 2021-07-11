use std::fs;
use std::path::Path;

use std::convert::TryInto;

use crate::error;

use error::Error::ConfigFileError;
use toml::Value;

pub struct Config {
    pub directories: Vec<String>,
    pub max_file_size: u64,
    pub file_tree: bool,
    pub tree_length: u8,
    pub tree_depth: u8,
}

impl Config {
    pub fn new(file_path: &Path) -> Result<Config, error::Error> {
        match file_path.extension() {
            Some(extension) => {
                if extension != "toml" {
                    return Err(ConfigFileError {
                        message: String::from("The configuration file is not a TOML-File"),
                    });
                }
            }
            None => {
                return Err(ConfigFileError {
                    message: String::from("The configuration file is not a TOML-File"),
                });
            }
        }

        let config_file: String = fs::read_to_string(&file_path)?;

        Ok(parse_toml_to_config(config_file)?)
    }
}

fn parse_toml_to_config(config_file: String) -> Result<Config, error::Error> {
    let parsed_toml: Value = toml::from_str(&config_file)?;

    let directories: Vec<String> = match parsed_toml.get("directories") {
        Some(v) => {
            let mut dirs: Vec<String> = Vec::new();

            for entry in v.as_array().unwrap().into_iter() {
                dirs.push(entry.to_string().replace("\"", ""));
            }

            dirs
        }
        None => {
            return Err(ConfigFileError {
                message: String::from("Your configuration does not have a \"directories\" key."),
            });
        }
    };

    let max_file_size: u64 = match parsed_toml.get("max_file_size") {
        Some(v) => v.as_integer().unwrap() as u64,
        None => {
            eprintln!(
                "Warning: Your configuration does not have a \"max_file_size\" key, \
                defaulting to 75 MB (75000000)."
            );

            75000000
        }
    };

    let file_tree: bool = match parsed_toml.get("file_tree") {
        Some(value) => value.as_bool().unwrap(),
        None => false,
    };

    let mut tree_length: u8 = 0;
    let mut tree_depth: u8 = 0;

    if file_tree {
        tree_length = match parsed_toml.get("tree_length") {
            Some(v) => match v.as_integer().unwrap().try_into() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!(
                        "Warning: The value of subdirectory_report_count could not \
                        be converted to an unsigned 8-bit integer. Defaulting to 3"
                    );

                    3
                }
            },
            None => 0,
        };

        tree_depth = match parsed_toml.get("tree_depth") {
            Some(v) => match v.as_integer().unwrap().try_into() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!(
                        "Warning: The value of file_report_count could not \
                        be converted to an unsigned 8-bit integer. Defaulting to 3"
                    );

                    3
                }
            },
            None => 0,
        };
    }

    Ok(Config {
        directories,
        max_file_size,
        file_tree,
        tree_length,
        tree_depth,
    })
}
