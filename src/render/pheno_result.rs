use std::fmt::{Display, Formatter};
use crate::phenotype::pheno_sim::Binary;

pub(crate) enum PhenoResult<'a> {
    Quantitative(f64),
    Case(&'a Binary),
    Control(&'a Binary),
}

impl Display for PhenoResult<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PhenoResult::Quantitative(number) => { write!(f, "{}", number) }
            PhenoResult::Case(binary) => { write!(f, "{}", binary.case) }
            PhenoResult::Control(binary) => { write!(f, "{}", binary.control) }
        }
    }
}