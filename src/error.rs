use std::io;
use std::num;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    #[error("Failed to parse integer: {0}")]
    ParseError(#[from] num::ParseIntError),
    #[error("TOML deserialization error: {0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error("Too many blocks. Found {count}, max count is {max_allowed}.")]
    TooManyWordsError { count: usize, max_allowed: u8 },
}
