use noodles::vcf::record::genotype::field::value::Genotype;

pub(crate) struct GenotypeStats {}

impl GenotypeStats {
    pub(crate) fn new(genotype: Genotype) -> GenotypeStats {
        for allele in &*genotype {


        }

        GenotypeStats {}
    }
}