use crate::config::VcfConfig;
use crate::error::Error;
use crate::{phenotype, sim};
use crate::read::{read_vcf_file, read_vcf_stdin};

pub(crate) fn process_vcf(config: &VcfConfig) -> Result<(), Error> {
    println!("Loading phenotypes from {}", config.phenotype_file);
    let phenotypes = phenotype::load::load(&config.phenotype_file)?;
    match &config.inputs {
        None => {
            let sim = read_vcf_stdin(&phenotypes)?;
            sim::io::write(&sim, &config.output)?;
        }
        Some(inputs) => {
            let mut inputs_iter = inputs.iter();
            match inputs_iter.next() {
                None => {
                    return Err(Error::from("Need to specify at least one input file."));
                }
                Some(input) => {
                    println!("Next reading {}", input);
                    let mut sim_all = read_vcf_file(input, &phenotypes)?;
                    println!("File: {}", sim_all.create_summary());
                    for input in inputs_iter {
                        println!("Next reading {}", input);
                        let sim_input = read_vcf_file(input, &phenotypes)?;
                        println!("File: {}", sim_input.create_summary());
                        sim_all = sim_all.try_add(&sim_input)?;
                        println!("All : {}", sim_all.create_summary());
                    }
                    sim::io::write(&sim_all, &config.output)?;
                }
            }
        }
    }
    Ok(())
}
