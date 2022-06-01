use noodles::core::{Position, Region};
use rand::Rng;

const MAX_POS: usize = 1 << 32;

pub(crate) struct RegionIterGen {
    region_size: usize,
    step_size_max: usize,
}

pub(crate) struct RegionIter {
    chrom: String,
    cursor: usize,
    region_size: usize,
    step_size_max: usize,
}

impl RegionIterGen {
    pub(crate) fn new(region_size: usize, step_size_max: usize) -> RegionIterGen {
        RegionIterGen { region_size, step_size_max }
    }
    pub(crate) fn new_region_iter(&self, chrom: String) -> RegionIter {
        RegionIter::new(chrom, self.region_size, self.step_size_max)
    }
}

impl RegionIter {
    pub(crate) fn new(chrom: String, region_size: usize, step_size_max: usize) -> RegionIter {
        let cursor = 1usize;
        RegionIter { chrom, cursor, region_size, step_size_max }
    }
}

impl Iterator for RegionIter {
    type Item = Region;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::thread_rng();
        self.cursor += rng.gen_range(0..self.step_size_max);
        let start = Position::try_from(self.cursor).unwrap();
        self.cursor += self.region_size;
        let end = Position::try_from(self.cursor).unwrap();
        if self.cursor < MAX_POS {
            Some(Region::new(self.chrom.clone(), start..end))
        } else {
            None
        }
    }
}

