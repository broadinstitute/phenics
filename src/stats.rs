pub(crate) mod sample_stats;
pub(crate) mod genotype_stats;

use crate::hash_key::HashKey;
use std::collections::HashMap;
use crate::error::Error;
use crate::stats::genotype_stats::GenotypeStats;
use crate::locus::Locus;
use crate::stats::sample_stats::SampleStats;

pub(crate) struct Stats {
    keys: Vec<HashKey>,
    sample_ids: HashMap<HashKey, String>,
    sample_stats: HashMap<HashKey, SampleStats>,
    n_records: u64,
}

impl Stats {
    pub(crate) fn new(sample_ids: Vec<String>) -> Stats {
        let mut keys = Vec::<HashKey>::new();
        let mut sample_ids_map = HashMap::<HashKey, String>::new();
        let mut sample_stats = HashMap::<HashKey, SampleStats>::new();
        for sample_id in sample_ids.into_iter() {
            let key = HashKey::from(sample_id.as_str());
            keys.push(key);
            sample_ids_map.insert(key, sample_id);
            sample_stats.insert(key, SampleStats::new());
        }
        let n_records = 0u64;
        Stats { keys, sample_ids: sample_ids_map, sample_stats, n_records }
    }
    pub(crate) fn add_genotypes(&mut self, locus: &Locus,
                                genotype_stats_list: Vec<Option<GenotypeStats>>)
                                -> Result<(), Error> {
        self.check_same_size_as_samples(&genotype_stats_list, locus,
                                        "genotypes")?;
        self.n_records += 1;
        Ok(())
    }
    pub(crate) fn merge_stats(&mut self, o_stats: Stats) {
        let Stats { keys, mut sample_ids, mut sample_stats, n_records } = o_stats;
        for key in keys.into_iter() {
            if self.sample_ids.contains_key(&key) {
                if let Some(self_sample_stats) =
                self.sample_stats.get_mut(&key) {
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
                    self.sample_stats.insert(key, o_sample_stats)
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