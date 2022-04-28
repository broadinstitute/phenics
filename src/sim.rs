pub(crate) mod sample_sim;
pub(crate) mod genotype_sim;
pub(crate) mod allele_sim;

use crate::hash_key::HashKey;
use std::collections::HashMap;
use crate::error::Error;
use crate::sim::genotype_sim::GenotypeSim;
use crate::locus::Locus;
use crate::sim::sample_sim::SampleSim;
use crate::sim::allele_sim::AlleleSim;

pub(crate) struct Sim {
    keys: Vec<HashKey>,
    sample_ids: HashMap<HashKey, String>,
    sample_sims: HashMap<HashKey, SampleSim>,
    n_records: u64,
}

impl Sim {
    pub(crate) fn new(sample_ids: Vec<String>, n_phenotypes: usize) -> Sim {
        let mut keys = Vec::<HashKey>::new();
        let mut sample_ids_map = HashMap::<HashKey, String>::new();
        let mut sample_stats = HashMap::<HashKey, SampleSim>::new();
        for sample_id in sample_ids.into_iter() {
            let key = HashKey::from(sample_id.as_str());
            keys.push(key);
            sample_ids_map.insert(key, sample_id);
            sample_stats.insert(key, SampleSim::new(n_phenotypes));
        }
        let n_records = 0u64;
        Sim { keys, sample_ids: sample_ids_map, sample_sims: sample_stats, n_records }
    }
    pub(crate) fn add_genotypes(&mut self, genotype_sims: &[Option<GenotypeSim>], locus: &Locus,
                                allele_sims: &Vec<AlleleSim>)
                                -> Result<(), Error> {
        self.check_same_size_as_samples(genotype_sims, locus, "genotypes")?;
        for (i_sample, genotype_sim) in genotype_sims.iter().enumerate() {
            let sample_key = &self.keys[i_sample];
            let sample_sim = self.sample_sims.get_mut(sample_key).unwrap();
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
    pub(crate) fn merge_stats(&mut self, o_stats: Sim) {
        let Sim { keys, mut sample_ids,
            sample_sims: mut sample_stats, n_records } = o_stats;
        for key in keys.into_iter() {
            if self.sample_ids.contains_key(&key) {
                if let Some(self_sample_stats) =
                self.sample_sims.get_mut(&key) {
                    if let Some(o_sample_stats) = sample_stats.remove(&key) {
                        self_sample_stats.merge(&o_sample_stats);
                    }
                }
            } else {
                sample_ids.remove(&key).map(|sample_id| {
                    self.keys.push(key);
                    self.sample_ids.insert(key, sample_id)
                });
                sample_stats.remove(&key).map(|o_sample_stats| {
                    self.sample_sims.insert(key, o_sample_stats)
                });
            }
        }
        self.n_records += n_records;
    }
    pub(crate) fn check_same_size_as_samples<T>(&self, items: &[T], locus: &Locus, item_type: &str)
                                                -> Result<(), Error> {
        if items.len() == self.keys.len() {
            Ok(())
        } else {
            Err(Error::from(
                format!("At {}, got {} {}, but have {} samples.", locus, items.len(),
                        item_type, self.keys.len())
            ))
        }
    }
    pub(crate) fn n_samples(&self) -> usize {
        self.keys.len()
    }
    pub(crate) fn n_records(&self) -> u64 {
        self.n_records
    }
    pub(crate) fn create_summary(&self) -> String {
        format!("{} samples, {} records.", self.n_samples(), self.n_records())
    }
}