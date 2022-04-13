use std::fmt::{Display, Formatter};

pub enum Error {
    Clap(clap::Error)
}

impl From<clap::Error> for Error {
    fn from(clap_error: clap::Error) -> Self {
        Error::Clap(clap_error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Clap(clap_error) => { writeln!(f, "{}", clap_error) }
        }
    }
}