use crate::config::GcsSampleConfig;
use crate::{Error, phenotype, tabix};
use crate::records::SimProcessor;
use crate::region_iter::RegionIterGen;
use crate::sim::Sim;
use crate::sim;

pub(crate) fn sample(config: &GcsSampleConfig) -> Result<(), Error> {
    println!("Loading phenotypes from {}", config.phenotype_file);
    let phenotypes = phenotype::load::load(&config.phenotype_file)?;
    println!("Now processing file {} with index {}.", config.data, config.index);
    let vcf_header = tabix::read_vcf_header(&config.data)?;
    let sample_ids: Vec<String> = vcf_header.sample_names().iter().map(String::from).collect();
    let mut sim = Sim::new(sample_ids, &phenotypes);
    let data = &config.data;
    let index = &config.index;
    let mut sim_processor = SimProcessor::new(&mut sim, &phenotypes);
    let region_iter_gen = RegionIterGen::new(config.region_size, config.step_size_max);
    let n_records =
        tabix::sample_regions(data, index, &mut sim_processor,
                              &region_iter_gen)?;
    println!("Read {} records", n_records);
    sim::io::write(&sim, &config.output)?;
    Ok(())
}

