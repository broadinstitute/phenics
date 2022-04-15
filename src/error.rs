use std::fmt::{Display, Formatter};
use std::io;
use noodles::vcf;

pub enum Error {
    Phenics(String),
    Clap(clap::Error),
    IO(io::Error),
    VcfHeaderParse(vcf::header::ParseError),
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

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self { Error::IO(io_error) }
}

impl From<vcf::header::ParseError> for Error {
    fn from(parse_error: vcf::header::ParseError) -> Self { Error::VcfHeaderParse(parse_error) }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Phenics(message) => { writeln!(f, "{}", message) }
            Error::Clap(clap_error) => { writeln!(f, "{}", clap_error) }
            Error::IO(io_error) => { writeln!(f, "{}", io_error) }
            Error::VcfHeaderParse(parse_error) => { writeln!(f, "{}", parse_error) }
        }
    }
}