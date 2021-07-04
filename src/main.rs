mod config;
mod error;
mod search;

use std::{collections::HashMap, env, path::PathBuf};

use config::Config;

fn main() {
    let mut arguments: env::Args = env::args();
    arguments.next();

    let config_file_name: PathBuf = match arguments.next() {
        Some(v) => PathBuf::from(v),
        None => {
            let mut config_file = dirs::config_dir().unwrap();
            config_file.push("ds-utils.toml");

            config_file
        }
    };

    let config: Config = match Config::new(&config_file_name) {
        Ok(config) => config,
        Err(err) => {
            let config_file_name = config_file_name.display();
            match err {
                error::Error::IoError(err) => {
                    eprintln!(
                        "Could not parse configuration file {}: {}",
                        config_file_name, err
                    );
                }
                _ => {
                    eprintln!(
                        "Unexpected error while parsing configuration file {}.",
                        config_file_name
                    );
                }
            }

            std::process::exit(1);
        }
    };

    let directories_found: HashMap<&str, u64> = search::search_target_directories(&config);

    for (directory, directory_size) in directories_found {
        println!(
            "{} exceeds the max size (size: {}, maximum allowed size: {})",
            directory, directory_size, &config.max_file_size
        );
    }
}
