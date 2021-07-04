use std::io;

use toml::de;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    TomlError(#[from] de::Error),

    #[error("Logic error: {message:?}")]
    ConfigFileError { message: String },
}
