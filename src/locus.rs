use noodles::vcf::record::{Chromosome, Position};
use std::fmt::{Display, Formatter};

pub(crate) struct Locus {
    chrom: String,
    pos: u32,
}

impl Locus {
    pub(crate) fn new(chromosome: &Chromosome, position: &Position) -> Locus {
        let chrom = format!("{}", chromosome);
        let pos = i32::from(*position) as u32;
        Locus { chrom, pos }
    }
}

impl Display for Locus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.chrom, self.pos)
    }
}