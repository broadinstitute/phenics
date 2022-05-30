use std::fmt::{Display, Formatter, Debug};
use std::io;
use noodles::vcf;
use noodles::vcf::record::genotypes::genotype::GenotypeError;
use rand::distributions::WeightedError;
use rand_distr::NormalError;
use std::num::{ParseIntError, ParseFloatError};
use reqwest::header::ToStrError;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Phenics,
    Clap,
    IO,
    VcfHeaderParse,
    Genotype,
    Weighted,
    Normal,
    ParseInt,
    ParseFloat,
    Reqwest,
    ToStr,
    GCAuth,
    Unknown,
}

#[derive(Clone)]
pub struct Error {
    kind: ErrorKind,
    message: String,
    sub_error: Option<Box<Error>>,
}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Error {
        let sub_error: Option<Box<Error>> = None;
        Error { kind, message, sub_error }
    }
    pub fn new_wrap(kind: ErrorKind, message: String, sub_error: Option<Box<Error>>) -> Error {
        Error { kind, message, sub_error }
    }
    pub fn from_error(kind: ErrorKind, error: &dyn std::error::Error) -> Error {
        let message = error.to_string();
        let sub_error =
            error.source().map(Error::from_unknown_error).map(Box::new);
        Error { kind, message, sub_error }
    }
    pub fn from_unknown_error(error: &dyn std::error::Error) -> Error {
        let kind = ErrorKind::Unknown;
        Error::from_error(kind, error)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.sub_error {
            None => { None }
            Some(sub_error) => { Some(sub_error)}
        }
    }
    fn cause(&self) -> Option<&dyn std::error::Error> { self.source() }
}

pub(crate) fn none_to_error<T>(option: Option<T>, message: &str) -> Result<T, Error> {
    option.ok_or_else(|| { Error::from(message) })
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error::new(ErrorKind::Phenics, String::from(message))
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error::new(ErrorKind::Phenics, message)
    }
}

impl From<clap::Error> for Error {
    fn from(clap_error: clap::Error) -> Self {
        Error::from_error(ErrorKind::Clap, &clap_error)
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Error::from_error(ErrorKind::IO, &io_error)
    }
}

impl From<vcf::header::ParseError> for Error {
    fn from(parse_error: vcf::header::ParseError) -> Self {
        Error::from_error(ErrorKind::VcfHeaderParse, &parse_error)
    }
}

impl From<GenotypeError> for Error {
    fn from(genotype_error: GenotypeError) -> Self {
        Error::from_error(ErrorKind::Genotype, &genotype_error)
    }
}

impl From<WeightedError> for Error {
    fn from(weighted_error: WeightedError) -> Self {
        Error::from_error(ErrorKind::Weighted, &weighted_error)
    }
}

impl From<NormalError> for Error {
    fn from(normal_error: NormalError) -> Self {
        Error::from_error(ErrorKind::Normal, &normal_error)
    }
}

impl From<ParseIntError> for Error {
    fn from(parse_int_error: ParseIntError) -> Self {
        Error::from_error(ErrorKind::ParseInt, &parse_int_error)
    }
}

impl From<ParseFloatError> for Error {
    fn from(parse_float_error: ParseFloatError) -> Self {
        Error::from_error(ErrorKind::ParseFloat, &parse_float_error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(reqwest_error: reqwest::Error) -> Self {
        Error::from_error(ErrorKind::Reqwest, &reqwest_error)
    }
}

impl From<ToStrError> for Error {
    fn from(to_str_error: ToStrError) -> Self {
        Error::from_error(ErrorKind::ToStr, &to_str_error)
    }
}

impl From<google_cloud_auth::error::Error> for Error {
    fn from(gc_auth_error: google_cloud_auth::error::Error) -> Self {
        Error::from_error(ErrorKind::GCAuth, &gc_auth_error)
    }
}

impl ErrorKind {
    pub fn as_str(&self) -> &str {
        match self {
            ErrorKind::Phenics => { "Phenics" }
            ErrorKind::Clap => { "Clap" }
            ErrorKind::IO => { "IO" }
            ErrorKind::VcfHeaderParse => { "VcfHeaderParse" }
            ErrorKind::Genotype => { "Genotype" }
            ErrorKind::Weighted => { "Weighted" }
            ErrorKind::Normal => { "Normal" }
            ErrorKind::ParseInt => { "ParseInt"}
            ErrorKind::ParseFloat => { "ParseFloat" }
            ErrorKind::Reqwest => { "Reqwest" }
            ErrorKind::ToStr => { "ToStr"}
            ErrorKind::GCAuth => { "GCAuth"}
            ErrorKind::Unknown => { "[unknown error type]"}
        }
    }
}

impl Error {
    pub(crate) fn into_io_error(self) -> io::Error {
        io::Error::new(io::ErrorKind::Other, self)
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.sub_error {
            None => { writeln!(f, "{}: {}", self.kind, self.message) }
            Some(sub_error) => {
                write!(f, "{}: {}: {}", self.kind, self.message, sub_error)
            }
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}