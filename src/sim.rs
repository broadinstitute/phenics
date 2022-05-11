pub(crate) mod sample_sim;
pub(crate) mod genotype_sim;
pub(crate) mod allele_sim;
pub(crate) mod io;

use crate::error::Error;
use crate::sim::genotype_sim::GenotypeSim;
use crate::locus::Locus;
use crate::sim::sample_sim::SampleSim;
use crate::sim::allele_sim::AlleleSim;
use crate::phenotype::Phenotype;
use crate::stats::Stats;
use rand_distr::Normal;
use rand::prelude::{Distribution, ThreadRng};
use crate::render::sample_result::SampleResult;
use crate::render::pheno_result::PhenoResult;
use crate::phenotype::pheno_sim::{Category, Binary};

pub(crate) struct Sim {
    phenotype_names: Vec<String>,
    sample_sims: Vec<SampleSim>,
    n_records: u64,
}

impl Sim {
    pub(crate) fn new(sample_ids: Vec<String>, phenotypes: &[Phenotype]) -> Sim {
        let phenotype_names: Vec<String> = phenotypes.iter().map(|phenotype| {
            String::from(&phenotype.name)
        }).collect();
        let sample_sims: Vec<SampleSim> =
            sample_ids.into_iter().map(|sample_id| {
                SampleSim::new(sample_id, phenotypes.len())
            }).collect();
        let n_records = 0u64;
        Sim { phenotype_names, sample_sims, n_records }
    }
    pub(crate) fn add_genotype_sim(&mut self, genotype_sim: &Option<GenotypeSim>, i_sample: usize,
                        allele_sims: &[AlleleSim]) {
        let sample_sim = &mut self.sample_sims[i_sample];
        match genotype_sim {
            None => { sample_sim.add_unknown_genotype() }
            Some(genotype_sim) => {
                for (i_allele, allele_sim) in allele_sims.iter().enumerate() {
                    sample_sim.add_allele_effects(genotype_sim, allele_sim, i_allele);
                }
            }
        }
    }
    pub(crate) fn count_record(&mut self) {
        self.n_records += 1;
    }
    pub(crate) fn try_add(&self, o_sim: &Sim) -> Result<Sim, Error> {
        let Sim {
            phenotype_names: o_phenotype_names,
            sample_sims: o_sample_sims, n_records: o_n_records
        } = o_sim;
        if self.phenotype_names.len() != o_phenotype_names.len() {
            return Err(Error::from(
                format!("Need to have the same phenotypes, but got {} phenotypes \
                versus {} phenotypes.", self.phenotype_names.len(), o_phenotype_names.len())));
        }
        let mut phenotype_names: Vec<String> = Vec::new();
        for (i, phenotype_name) in self.phenotype_names.iter().enumerate() {
            let o_phenotype_name = &o_phenotype_names[i];
            if phenotype_name.as_str().eq(o_phenotype_name.as_str()) {
                phenotype_names.push(phenotype_name.clone())
            } else {
                return Err(Error::from(
                    format!("Need to have the same phenotypes, but got '{}' versus '{}'.",
                            phenotype_name, o_phenotype_name)));
            }
        }
        let mut sample_sims: Vec<SampleSim> = Vec::new();
        for (i, self_sample_sim) in self.sample_sims.iter().enumerate() {
            sample_sims.push(self_sample_sim.try_add(&o_sample_sims[i])?)
        }
        let n_records = self.n_records + o_n_records;
        Ok(Sim { phenotype_names, sample_sims, n_records })
    }
    pub(crate) fn check_same_size_as_samples<T>(&self, items: &[T], locus: &Locus, item_type: &str)
                                                -> Result<(), Error> {
        if items.len() == self.sample_sims.len() {
            Ok(())
        } else {
            Err(Error::from(
                format!("At {}, got {} {}, but have {} samples.", locus, items.len(),
                        item_type, self.sample_sims.len())
            ))
        }
    }
    pub(crate) fn n_samples(&self) -> usize { self.sample_sims.len() }
    pub(crate) fn n_records(&self) -> u64 {
        self.n_records
    }
    pub(crate) fn create_summary(&self) -> String {
        format!("{} samples, {} records.", self.n_samples(), self.n_records())
    }
    fn new_env_distributions(&self, phenotypes: &[Phenotype], stats: &Stats)
                             -> Result<Vec<Normal<f64>>, Error> {
        let mut distributions: Vec<Normal<f64>> = Vec::new();
        let gen_variances = stats.variances();
        for (i, phenotype) in phenotypes.iter().enumerate() {
            let h2 = phenotype.sim.heritability;
            let std_dev = (gen_variances[i] * (1.0 - h2) / h2).sqrt();
            distributions.push(Normal::new(0f64, std_dev)?);
        }
        Ok(distributions)
    }
    fn new_liabilities(&self, env_distributions: &[Normal<f64>]) -> Vec<Vec<f64>> {
        let mut liabilities: Vec<Vec<f64>> = Vec::new();
        for sample_sim in &self.sample_sims {
            let mut sample_liabilities: Vec<f64> = Vec::new();
            for (i, gen_effect) in sample_sim.effects.iter().enumerate() {
                let env_effect =
                    env_distributions[i].sample::<ThreadRng>(&mut rand::thread_rng());
                let liability = gen_effect + env_effect;
                sample_liabilities.push(liability);
            };
            liabilities.push(sample_liabilities);
        };
        liabilities
    }
    fn new_sample_results(&self, liabilities: &[Vec<f64>], phenotypes: &[Phenotype])
                          -> Vec<SampleResult> {
        let mut sample_results: Vec<SampleResult> =
            self.sample_sims.iter().map(|sample_sim| {
                SampleResult::new(sample_sim.id.clone(), Vec::<PhenoResult>::new())
            }).collect();
        for (i_pheno, phenotype) in phenotypes.iter().enumerate() {
            match &phenotype.sim.category {
                Category::Quantitative => {
                    for (i_sample, sample_result) in
                    sample_results.iter_mut().enumerate() {
                        let liability = liabilities[i_sample][i_pheno];
                        sample_result.pheno_results.push(PhenoResult::Quantitative(liability))
                    }
                }
                Category::Binary(binary) => {
                    let Binary { prevalence, .. } = binary;
                    let n_cases = ((sample_results.len() as f64) * prevalence) as usize;
                    let n_controls = sample_results.len() - n_cases;
                    let mut pheno_results: Vec<PhenoResult> = Vec::new();
                    for _ in 0..n_cases { pheno_results.push(PhenoResult::Case) }
                    for _ in 0..n_controls { pheno_results.push(PhenoResult::Control) }
                    let mut sorting_pheno_results = true;
                    while sorting_pheno_results {
                        sorting_pheno_results = false;
                        let mut case_min: Option<(usize, f64)> = None;
                        let mut control_max: Option<(usize, f64)> = None;
                        for (i_sample, pheno_result)
                        in pheno_results.iter().enumerate() {
                            let liability = liabilities[i_sample][i_pheno];
                            match pheno_result {
                                PhenoResult::Quantitative(_) => {}
                                PhenoResult::Case => {
                                    match case_min {
                                        None => { case_min = Some((i_sample, liability)) }
                                        Some((_, min_liability)) => {
                                            if liability < min_liability {
                                                case_min = Some((i_sample, liability))
                                            }
                                        }
                                    }
                                }
                                PhenoResult::Control => {
                                    match control_max {
                                        None => { control_max = Some((i_sample, liability)) }
                                        Some((_, max_liability)) => {
                                            if liability > max_liability {
                                                control_max = Some((i_sample, liability))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if let Some((i_case_min, _)) = case_min {
                            if let Some((i_control_max, _)) = control_max {
                                if i_case_min < i_control_max {
                                    pheno_results[i_case_min] = PhenoResult::Control;
                                    pheno_results[i_control_max] = PhenoResult::Case;
                                    sorting_pheno_results = true;
                                }
                            }
                        }
                    }
                    for (i_sample, pheno_result) in
                    pheno_results.drain(..).enumerate() {
                        sample_results[i_sample].pheno_results.push(pheno_result);
                    }
                }
            }
        }
        sample_results
    }
    pub(crate) fn render_phenotypes(&self, phenotypes: &[Phenotype])
                                        -> Result<Vec<SampleResult>, Error> {
        let mut stats = Stats::new(phenotypes.len());
        for sample_sim in &self.sample_sims {
            stats.add(&sample_sim.effects)?;
        }
        let env_distributions = self.new_env_distributions(phenotypes, &stats)?;
        let liabilities = self.new_liabilities(&env_distributions);
        let sample_results = self.new_sample_results(&liabilities, phenotypes);
        Ok(sample_results)
    }
}