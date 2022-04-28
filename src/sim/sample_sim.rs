use crate::sim::genotype_sim::GenotypeSim;
use crate::sim::allele_sim::AlleleSim;

pub(crate) struct SampleSim {
    effects: Vec<f64>,
    n_unknown_genotypes: u64,
    n_unknown_alleles: u64,
}

impl SampleSim {
    pub(crate) fn new(n_phenotypes: usize) -> SampleSim {
        let n_unknown_genotypes = 0u64;
        let n_unknown_alleles = 0u64;
        let effects: Vec<f64> = vec![0f64; n_phenotypes];
        SampleSim { n_unknown_genotypes, n_unknown_alleles, effects }
    }
    pub(crate) fn merge(&mut self, o_stats: &SampleSim) {
        self.n_unknown_genotypes += o_stats.n_unknown_genotypes;
        self.n_unknown_alleles += o_stats.n_unknown_alleles;
    }
    pub(crate) fn add_unknown_genotype(&mut self) {
        self.n_unknown_genotypes += 1;
    }
    pub(crate) fn add_allele_effects(&mut self, genotype: &GenotypeSim, allele: &AlleleSim) {
        for (i, dosage) in genotype.dosages.iter().enumerate() {
            self.effects[i] += (*dosage as f64) * allele.effects[i];
        }
        self.n_unknown_alleles += genotype.n_unknown_alleles;
    }
}