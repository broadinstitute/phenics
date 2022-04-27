mod tokenize;
mod treeize;
mod analyze;

use crate::phenotype::Phenotype;
use crate::error::Error;
use std::fmt::{Display, Formatter};

fn cannot_parse(string: &str, problem: &str) -> Error {
    Error::from(format!("Cannot parse {}: {}", string, problem))
}

pub(crate) fn parse(string: &str) -> Result<Phenotype, Error> {
    let mut split_by_eq = string.split('=');
    let name = String::from(split_by_eq.next().ok_or_else(|| {
        cannot_parse(string, "no '='")
    })?);
    let definition =
        split_by_eq.next().ok_or_else(|| { cannot_parse(string, "no '='") })?;
    if split_by_eq.next().is_some() {
        return Err(cannot_parse(string, "More than one '='."));
    }
    let tokens = tokenize::tokenize(definition)?;
    let call = treeize::treeize(tokens)?;
    let sim = analyze::analyze(call)?;
    Ok(Phenotype::new(name, sim))
}

pub(self) enum Value {
    String(String),
    Number(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(string) => { write!(f, "{}", string) }
            Value::Number(number) => { write!(f, "{}", number) }
        }
    }
}



