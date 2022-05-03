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
    pub(crate) fn add_genotypes(&mut self, genotype_sims: &[Option<GenotypeSim>], locus: &Locus,
                                allele_sims: &Vec<AlleleSim>)
                                -> Result<(), Error> {
        self.check_same_size_as_samples(genotype_sims, locus, "genotypes")?;
        for (i_sample, genotype_sim) in genotype_sims.iter().enumerate() {
            let sample_sim = self.sample_sims.get_mut(i_sample).unwrap();
            match genotype_sim {
                None => { sample_sim.add_unknown_genotype() }
                Some(genotype_sim) => {
                    for allele_sim in allele_sims {
                        sample_sim.add_allele_effects(genotype_sim, allele_sim);
                    }
                }
            }
        }
        self.n_records += 1;
        Ok(())
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
            for (i, gen_effect) in sample_sim.effects.iter().enumerate(){
                let env_effect =
                    env_distributions[i].sample::<ThreadRng>(&mut rand::thread_rng());
                let liability = gen_effect + env_effect;
                sample_liabilities.push(liability);
            };
            liabilities.push(sample_liabilities);
        };
        liabilities
    }
    fn new_sample_results(&self, liabilities: &[Vec<f64>]) -> Vec<SampleResult> {
        todo!()
    }
    pub(crate) fn render_phenotypes(&self, phenotypes: &[Phenotype])
        -> Result<Vec<SampleResult>, Error> {
        let mut stats = Stats::new(phenotypes.len());
        for sample_sim in &self.sample_sims {
            stats.add(&sample_sim.effects)?;
        }
        let env_distributions = self.new_env_distributions(phenotypes, &stats)?;
        let liabilities = self.new_liabilities(&env_distributions);
        let sample_results = self.new_sample_results(&liabilities);
        Ok(sample_results)
    }
}