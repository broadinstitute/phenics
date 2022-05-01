mod tokenize;
mod treeize;
mod analyze;

use crate::phenotype::Phenotype;
use crate::error::Error;
use std::fmt::{Display, Formatter};
use crate::phenotype::pheno_sim::PhenoSim;

fn cannot_parse(string: &str, problem: &str) -> Error {
    Error::from(format!("Cannot parse {}: {}", string, problem))
}

pub(crate) fn parse(string: &str) -> Result<Vec<Phenotype>, Error> {
    let mut split_by_eq = string.split('=');
    let name_string = String::from(split_by_eq.next().ok_or_else(|| {
        cannot_parse(string, "no '='")
    })?);
    let definition =
        split_by_eq.next().ok_or_else(|| { cannot_parse(string, "no '='") })?;
    if split_by_eq.next().is_some() {
        return Err(cannot_parse(string, "More than one '='."));
    }
    let sim = parse_sim(definition)?;
    let phenotypes =
        parse_names(&name_string)?.into_iter()
            .map(|name| { Phenotype::new(name, sim.clone()) }).collect();
    Ok(phenotypes)
}

fn parse_names(name_string: &str) -> Result<Vec<String>, Error> {
    if let Some(stripped) = name_string.strip_suffix(']') {
        let mut split = stripped.split('[');
        let base_name =
            split.next().ok_or_else(|| {
                Error::from(format!("Malformed name definition '{}'.", name_string))
            })?;
        let number: usize = split.next().ok_or_else(|| {
            Error::from(format!("Malformed name definition '{}'.", name_string))
        })?.parse()?;
        Ok((0..number).map(|i| { format!("{}{}", base_name, i) }).collect())
    } else {
        Ok(vec![String::from(name_string)])
    }
}

fn parse_sim(definition: &str) -> Result<PhenoSim, Error> {
    let tokens = tokenize::tokenize(definition)?;
    let call = treeize::treeize(tokens)?;
    let sim = analyze::analyze(call)?;
    Ok(sim)
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



