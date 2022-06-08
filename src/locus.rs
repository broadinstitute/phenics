use noodles::vcf::record::{Chromosome, Position};
use std::fmt::{Display, Formatter};

pub(crate) struct Locus {
    chrom: String,
    pos: usize,
}

impl Locus {
    pub(crate) fn new(chromosome: &Chromosome, position: &Position) -> Locus {
        let chrom = format!("{}", chromosome);
        let pos = usize::from(*position);
        Locus { chrom, pos }
    }
}

impl Display for Locus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.chrom, self.pos)
    }
}