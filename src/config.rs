use std::fs;
use std::path::Path;

use crate::error;

use error::Error::ConfigFileError;
use toml::Value;

pub struct Config {
    pub directories: Vec<String>,
    pub max_file_size: u64,
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

        let (directories, max_file_size) = parse_toml(config_file)?;

        Ok(Config {
            directories,
            max_file_size,
        })
    }
}

fn parse_toml(config_file: String) -> Result<(Vec<String>, u64), error::Error> {
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

    Ok((directories, max_file_size))
}
