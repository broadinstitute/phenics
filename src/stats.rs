use crate::hash_key::HashKey;
use std::collections::HashMap;

pub(crate) struct Stats {
    keys: Vec<HashKey>,
    sample_ids: HashMap<HashKey, String>,
}

impl Stats {
    pub(crate) fn new(sample_ids: Vec<String>) -> Stats {
        let mut keys = Vec::<HashKey>::new();
        let mut sample_ids_map = HashMap::<HashKey, String>::new();
        for sample_id in sample_ids.into_iter() {
            let key = HashKey::from(sample_id.as_str());
            keys.push(key);
            sample_ids_map.insert(key, sample_id);
        }
        Stats { keys, sample_ids: sample_ids_map }
    }
    pub(crate) fn add(&mut self, o_stats: Stats) {
        let mut sample_ids = o_stats.sample_ids;
        for key in o_stats.keys.into_iter() {
            if self.sample_ids.contains_key(&key) {
                // no-op, for now
            } else {
                sample_ids.remove(&key).map(|sample_id| {
                    self.keys.push(key);
                    self.sample_ids.insert(key, sample_id)
                });
            }
        }
    }
    pub(crate) fn create_summary(&self) -> String {
        format!("{} samples.", self.keys.len())
    }
}