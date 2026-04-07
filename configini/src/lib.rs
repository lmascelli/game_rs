pub use configini_derive::Configini;

#[derive(Debug)]
pub enum ConfiginiError {
    IoError(std::io::Error),
    ParseError(ini::ParseError),
}

impl From<std::io::Error> for ConfiginiError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

pub use ini::Ini;
