use crate::sim::Sim;
use crate::error::Error;
use noodles::{bgzf, vcf};
use fs_err::File;
use noodles::vcf::Header;
use crate::sim::genotype_sim::GenotypeSim;
use crate::locus::Locus;
use crate::sim::allele_sim::AlleleSim;
use crate::phenotype::Phenotype;
use std::io::{stdin, BufRead, BufReader};

pub(crate) fn read_vcf_file(file: &str, phenotypes: &[Phenotype]) -> Result<Sim, Error> {
    let reader = bgzf::Reader::new(File::open(file)?);
    read_vcf_reader(reader, phenotypes)
}

pub(crate) fn read_vcf_stdin(phenotypes: &[Phenotype]) -> Result<Sim, Error> {
    let reader = BufReader::new(stdin());
    read_vcf_reader(reader, phenotypes)
}

fn read_vcf_reader<R: BufRead>(reader: R, phenotypes: &[Phenotype]) -> Result<Sim, Error> {
    let mut vcf_reader = vcf::Reader::new(reader);
    let header = vcf_reader.read_header()?.parse::<Header>()?;
    let sample_ids: Vec<String> = header.sample_names().iter().map(String::from).collect();
    let mut sim = Sim::new(sample_ids, phenotypes);
    for record in vcf_reader.records(&header) {
        let record = record?;
        let genotypes = record.genotypes().genotypes()?;
        let allele_sims =
            record.alternate_bases().iter().map(|_alts| { AlleleSim::from_phenotypes(phenotypes) })
                .collect::<Vec<AlleleSim>>();
        let locus = Locus::new(record.chromosome(), &record.position());
        sim.check_same_size_as_samples(&genotypes, &locus, "genotypes")?;
        for (i_sample, genotype) in genotypes.iter().enumerate() {
            let genotype_sim =
                genotype.as_ref()
                    .map(|genotype| { GenotypeSim::new(genotype, allele_sims.len()) });
            sim.add_genotype_sim(&genotype_sim, i_sample, &allele_sims);
        }
        sim.count_record();
    }
    Ok(sim)
}