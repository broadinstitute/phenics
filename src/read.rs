use crate::sim::Sim;
use crate::error::Error;
use noodles::{bgzf, vcf};
use fs_err::File;
use noodles::vcf::Header;
use crate::phenotype::Phenotype;
use std::io::{stdin, BufRead, BufReader};
use crate::records::{SimProcessor, RecordProcessor};

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
    let mut sim_processor = SimProcessor::new(&mut sim, phenotypes);
    for record in vcf_reader.records(&header) {
        let record = record?;
        sim_processor.process_record(&record)?;
    }
    Ok(sim)
}
