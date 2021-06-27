use std::str::FromStr;
use std::io;

#[derive(Debug)]
pub enum RunConfig {
    Rust,
}

impl FromStr for RunConfig {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase()[..] {
            "rust" => Ok(RunConfig::Rust),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Error parsing RunConfig")),
        }
    }
}
