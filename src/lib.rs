use crate::config::get_config;
use crate::error::Error;
use crate::read::read_vcf;

mod config;
mod error;
mod stats;
mod read;
mod hash_key;
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
            println!("Next reading {}", input);
            let mut stats_all = read_vcf(input)?;
            println!("File: {}", stats_all.create_summary());
            for input in inputs_iter {
                println!("Next reading {}", input);
                let stats_input = read_vcf(input)?;
                println!("File: {}", stats_input.create_summary());
                stats_all.merge_stats(stats_input);
                println!("All : {}", stats_all.create_summary());
            }
        }
    }
    Ok(())
}