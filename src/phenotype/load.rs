use crate::error::Error;
use crate::phenotype::Phenotype;
use fs_err::File;
use std::io::{BufReader, BufRead};
use crate::phenotype::parse::parse;

pub(crate) fn load(file: &str) -> Result<Vec<Phenotype>, Error> {
    let mut phenotypes: Vec<Phenotype> = Vec::new();
    for line in BufReader::new(File::open(file)?).lines() {
        let line = line?;
        phenotypes.append(&mut parse(&line)?)

    };
    Ok(phenotypes)
}

