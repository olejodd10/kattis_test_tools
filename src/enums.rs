use std::str::FromStr;
use std::io;

#[derive(Debug)]
pub enum RunConfig {
    Rust,
    Haskell,
    Python,
}

impl FromStr for RunConfig {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase()[..] {
            "rust" => Ok(RunConfig::Rust),
            "haskell" => Ok(RunConfig::Haskell),
            "python" => Ok(RunConfig::Python),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Error parsing RunConfig")),
        }
    }
}
