use std::fmt::{Display, Formatter};

pub enum Error {
    Phenics(String),
    Clap(clap::Error),
}

pub(crate) fn none_to_error<T>(option: Option<T>, message: &str) -> Result<T, Error> {
    option.ok_or_else(|| { Error::from(message) })
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error::Phenics(String::from(message))
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error::Phenics(message)
    }
}

impl From<clap::Error> for Error {
    fn from(clap_error: clap::Error) -> Self {
        Error::Clap(clap_error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Phenics(message) => { writeln!(f, "{}", message) }
            Error::Clap(clap_error) => { writeln!(f, "{}", clap_error) }
        }
    }
}