use noodles::vcf::record::genotype::field::value::Genotype;

pub(crate) struct GenotypeSim {
    pub(crate) dosages: Vec<u8>,
    pub(crate) n_unknown_alleles: u64
}

impl GenotypeSim {
    pub(crate) fn new(genotype: Genotype, n_alt: usize) -> GenotypeSim {
        let mut dosages: Vec<u8> = vec![0; n_alt];
        let mut n_unknown_alleles: u64 = 0;
        for allele in &*genotype {
            if let Some(pos) = allele.position() {
                let i = pos - 1;
                dosages[i] += 1;
            } else {
                n_unknown_alleles += 1;
            }
        }
        GenotypeSim { dosages, n_unknown_alleles }
    }
}