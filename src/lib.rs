use crate::config::get_config;
use crate::error::Error;
use crate::read::read_vcf;

mod config;
mod error;
mod sim;
mod read;
mod locus;
mod phenotype;

pub fn run() -> Result<(), Error> {
    let config = get_config()?;
    let mut inputs_iter = config.inputs.iter();
    match inputs_iter.next() {
        None => {
            return Err(Error::from("Need to specify at least one input file."));
        }
        Some(input) => {
            println!("Loading phenotypes from {}", config.phenotype_file);
            let phenotypes = phenotype::load::load(&config.phenotype_file)?;
            println!("Next reading {}", input);
            let mut sim_all = read_vcf(input, &phenotypes)?;
            println!("File: {}", sim_all.create_summary());
            for input in inputs_iter {
                println!("Next reading {}", input);
                let sim_input = read_vcf(input, &phenotypes)?;
                println!("File: {}", sim_input.create_summary());
                sim_all = sim_all.try_add(&sim_input)?;
                println!("All : {}", sim_all.create_summary());
            }
        }
    }
    Ok(())
}