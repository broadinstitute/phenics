use crate::phenotype::pheno_sim::PhenoSim;

pub(crate) mod parse;
mod pheno_sim;

pub(crate) struct Phenotype {
    pub(crate) name: String,
    pub(crate) sim: PhenoSim
}

impl Phenotype {
    pub(crate) fn new(name: String, sim: PhenoSim) -> Phenotype {
        Phenotype { name, sim }
    }
}