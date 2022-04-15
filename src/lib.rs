use crate::config::get_config;
use crate::error::Error;
use crate::read::read_vcf;

mod config;
mod error;
mod stats;
mod read;
mod hash_key;

pub fn run() -> Result<(), Error> {
    let config = get_config()?;
    let mut inputs_iter = config.inputs.iter();
    match inputs_iter.next() {
        None => {
            return Err(Error::from("Need to specify at least one input file."));
        }
        Some(input) => {
            let mut stats = read_vcf(input)?;
            for input in inputs_iter {
                stats.add(read_vcf(input)?)?;
            }
        }
    }
    Ok(())
}