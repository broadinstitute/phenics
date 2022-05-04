use crate::phenotype::pheno_sim::Category;
use crate::error::Error;

pub(crate) enum PhenoResult {
    Quantitative(f64),
    Case,
    Control,
}

impl PhenoResult {
    pub(crate) fn to_string_for(&self, category: &Category) -> Result<String, Error> {
        match (self, category) {
            (PhenoResult::Quantitative(number), Category::Quantitative) => {
                Ok(format!("{}", number))
            }
            (PhenoResult::Case, Category::Binary(binary)) => {
                Ok(binary.case.clone())
            }
            (PhenoResult::Control, Category::Binary(binary)) => {
                Ok(binary.control.clone())
            }
            _ => {
                Err(Error::from("Phenotype result is of wrong category."))
            }
        }
    }
}

