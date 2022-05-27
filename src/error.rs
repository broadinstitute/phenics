use std::fmt::{Display, Formatter};
use std::io;
use noodles::vcf;
use noodles::vcf::record::genotypes::genotype::GenotypeError;
use rand::distributions::WeightedError;
use rand_distr::NormalError;
use std::num::{ParseIntError, ParseFloatError};

pub enum Error {
    Phenics(String),
    Clap(clap::Error),
    IO(io::Error),
    VcfHeaderParse(vcf::header::ParseError),
    Genotype(GenotypeError),
    Weighted(WeightedError),
    Normal(NormalError),
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
    Reqwest(reqwest::Error),
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

impl From<GenotypeError> for Error {
    fn from(genotype_error: GenotypeError) -> Self { Error::Genotype(genotype_error) }
}

impl From<WeightedError> for Error {
    fn from(weighted_error: WeightedError) -> Self { Error::Weighted(weighted_error) }
}

impl From<NormalError> for Error {
    fn from(normal_error: NormalError) -> Self { Error::Normal(normal_error) }
}

impl From<ParseIntError> for Error {
    fn from(parse_int_error: ParseIntError) -> Self { Error::ParseInt(parse_int_error) }
}

impl From<ParseFloatError> for Error {
    fn from(parse_float_error: ParseFloatError) -> Self { Error::ParseFloat(parse_float_error) }
}

impl From<reqwest::Error> for Error {
    fn from(reqwest_error: reqwest::Error) -> Self { Error::Reqwest(reqwest_error) }
}

fn wrap_as_io_error<E>(error: E) -> io::Error {
    io::
}

impl Error {
    pub(crate) fn to_io_error(self) -> io::Error {
        match self {
            Error::Phenics(phenics_error) => { }
            Error::Clap(_) => {}
            Error::IO(_) => {}
            Error::VcfHeaderParse(_) => {}
            Error::Genotype(_) => {}
            Error::Weighted(_) => {}
            Error::Normal(_) => {}
            Error::ParseInt(_) => {}
            Error::ParseFloat(_) => {}
            Error::Reqwest(_) => {}
        }
    }
}

impl Clone for Error {
    fn clone(&self) -> Self {
        match self {
            Error::Phenics(phenics_error) => { Error::Phenics(phenics_error.clone()) }
            Error::Clap(clap_error) => {
                Error::from(format!("clap::Error: {}", clap_error))
            }
            Error::IO(io_error) => {
                Error::from(format!("io::Error: {}", io_error))
            }
            Error::VcfHeaderParse(vcf_header) => {
                Error::VcfHeaderParse(vcf_header.clone())
            }
            Error::Genotype(genotype_error) => {
                Error::Genotype(genotype_error.clone())
            }
            Error::Weighted(weighted_error) => { Error::Weighted(*weighted_error) }
            Error::Normal(normal_error) => { Error::Normal(*normal_error) }
            Error::ParseInt(parse_int) => { Error::ParseInt(parse_int.clone()) }
            Error::ParseFloat(parse_float) => { Error::ParseFloat(parse_float.clone()) }
            Error::Reqwest(reqwest_error) => {
                Error::from(format!("reqwest::Error: {}", reqwest_error))
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Phenics(message) => { writeln!(f, "{}", message) }
            Error::Clap(clap_error) => { writeln!(f, "{}", clap_error) }
            Error::IO(io_error) => { writeln!(f, "{}", io_error) }
            Error::VcfHeaderParse(parse_error) => { writeln!(f, "{}", parse_error) }
            Error::Genotype(genotype_error) => { writeln!(f, "{}", genotype_error) }
            Error::Weighted(weighted_error) => { writeln!(f, "{}", weighted_error) }
            Error::Normal(normal_error) => { writeln!(f, "{}", normal_error) }
            Error::ParseInt(parse_int_error) => { writeln!(f, "{}", parse_int_error) }
            Error::ParseFloat(parse_float) => { writeln!(f, "{}", parse_float) }
            Error::Reqwest(reqwest_error) => { writeln!(f, "{}", reqwest_error) }
        }
    }
}