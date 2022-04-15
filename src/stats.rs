use crate::error::Error;

pub(crate) struct Stats {
    sample_ids: Vec<String>
}

impl Stats {
    pub(crate) fn new(sample_ids: Vec<String>) -> Stats {
        Stats { sample_ids }
    }
    pub(crate) fn add(&mut self, o_stats: Stats) -> Result<Stats, Error> {
        todo!()
    }
}