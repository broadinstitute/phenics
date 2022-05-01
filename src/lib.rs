use crate::config::{Config, get_config};
use crate::error::Error;

mod config;
mod error;
mod sim;
mod read;
mod locus;
mod phenotype;
mod vcf;
mod check;

pub fn run() -> Result<(), Error> {
    let config = get_config()?;
    match config {
        Config::Check(check_config) => { check::check(check_config) }
        Config::Vcf(vcf_config) => { vcf::process_vcf(&vcf_config) }
    }
}