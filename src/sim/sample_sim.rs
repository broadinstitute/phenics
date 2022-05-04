use crate::sim::genotype_sim::GenotypeSim;
use crate::sim::allele_sim::AlleleSim;
use crate::error::Error;

pub(crate) struct SampleSim {
    pub(crate) id: String,
    pub(crate) effects: Vec<f64>,
    pub(crate) n_unknown_genotypes: u64,
    pub(crate) n_unknown_alleles: u64,
}

impl SampleSim {
    pub(crate) fn new(id: String, n_phenotypes: usize) -> SampleSim {
        let n_unknown_genotypes = 0u64;
        let n_unknown_alleles = 0u64;
        let effects: Vec<f64> = vec![0f64; n_phenotypes];
        SampleSim { id, effects, n_unknown_genotypes, n_unknown_alleles }
    }
    pub(crate) fn try_add(&self, o_sample_sim: &SampleSim) -> Result<SampleSim, Error> {
        if self.id.as_str().ne(o_sample_sim.id.as_str()) {
            return Err(Error::from(
                format!("Sample ids needs to be equal, but are '{}' and '{}'.",
                        self.id, o_sample_sim.id)));
        }
        if self.effects.len() != o_sample_sim.effects.len() {
            return Err(Error::from(
                format!("Number of effects needs to be equal, but are '{}' and '{}'.",
                        self.effects.len(), o_sample_sim.effects.len())));
        }
        let id = self.id.clone();
        let effects: Vec<f64> =
            self.effects.iter().enumerate().map(|(i, effect)| {
                effect + o_sample_sim.effects[i]
            }).collect();
        let n_unknown_genotypes = self.n_unknown_genotypes + o_sample_sim.n_unknown_genotypes;
        let n_unknown_alleles = self.n_unknown_alleles + o_sample_sim.n_unknown_alleles;
        Ok(SampleSim { id, effects, n_unknown_genotypes, n_unknown_alleles })
    }
    pub(crate) fn add_unknown_genotype(&mut self) {
        self.n_unknown_genotypes += 1;
    }
    pub(crate) fn add_allele_effects(&mut self, genotype: &GenotypeSim, allele: &AlleleSim,
                                     i_allele: usize) {
        let dosage = genotype.dosages[i_allele];
        for (i, effect) in allele.effects.iter().enumerate() {
            self.effects[i] += (dosage as f64) * effect;
        }
        self.n_unknown_alleles += genotype.n_unknown_alleles;
    }
}