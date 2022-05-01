pub(crate) mod sample_sim;
pub(crate) mod genotype_sim;
pub(crate) mod allele_sim;
pub(crate) mod write;

use crate::error::Error;
use crate::sim::genotype_sim::GenotypeSim;
use crate::locus::Locus;
use crate::sim::sample_sim::SampleSim;
use crate::sim::allele_sim::AlleleSim;
use crate::phenotype::Phenotype;

pub(crate) struct Sim {
    phenotype_names: Vec<String>,
    sample_sims: Vec<SampleSim>,
    n_records: u64,
}

impl Sim {
    pub(crate) fn new(sample_ids: Vec<String>, phenotypes: &[Phenotype]) -> Sim {
        let phenotype_names: Vec<String> = phenotypes.iter().map(|phenotype|{
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
        let Sim { phenotype_names: o_phenotype_names,
            sample_sims: o_sample_sims, n_records: o_n_records } = o_sim;
        if self.phenotype_names.len() != o_phenotype_names.len() {
            return Err(Error::from(
                format!("Need to have the same phenotypes, but got {} phenotypes \
                versus {} phenotypes.", self.phenotype_names.len(), o_phenotype_names.len())))
        }
        let mut phenotype_names: Vec<String> = Vec::new();
        for (i, phenotype_name) in self.phenotype_names.iter().enumerate() {
            let o_phenotype_name = &o_phenotype_names[i];
            if phenotype_name.as_str().eq(o_phenotype_name.as_str()) {
                phenotype_names.push(phenotype_name.clone())
            } else {
                return Err(Error::from(
                    format!("Need to have the same phenotypes, but got '{}' versus '{}'.",
                            phenotype_name, o_phenotype_name)))

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
}