use crate::sim::Sim;
use crate::error::Error;
use noodles::{bgzf, vcf};
use fs_err::File;
use noodles::vcf::Header;
use crate::sim::genotype_sim::GenotypeSim;
use crate::locus::Locus;
use crate::sim::allele_sim::AlleleSim;
use crate::phenotype::Phenotype;

pub(crate) fn read_vcf(file: &str, phenotypes: &[Phenotype]) -> Result<Sim, Error> {
    let mut reader =
        vcf::Reader::new(bgzf::Reader::new(File::open(file)?));
    let header = reader.read_header()?.parse::<Header>()?;
    let sample_ids: Vec<String> = header.sample_names().iter().map(String::from).collect();
    let mut sim = Sim::new(sample_ids, phenotypes);
    for record in reader.records(&header) {
        let record = record?;
        let genotypes = record.genotypes().genotypes()?;
        let allele_sims =
            record.alternate_bases().iter().map(|_alts|{ AlleleSim::from_phenotypes(phenotypes) })
                .collect::<Vec<AlleleSim>>();
        let locus = Locus::new(record.chromosome(), &record.position());
        sim.check_same_size_as_samples(&genotypes, &locus, "genotypes")?;
        let mut genotype_sims = Vec::<Option<GenotypeSim>>::new();
        for genotype in genotypes {
            let genotype_stats =
                genotype.map(|genotype|{ GenotypeSim::new(genotype, allele_sims.len()) });
            genotype_sims.push(genotype_stats);
        }
        sim.add_genotypes(&genotype_sims, &locus, &allele_sims)?;
    }
    Ok(sim)
}