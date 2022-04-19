use crate::config::get_config;
use crate::error::Error;
use crate::read::read_vcf;
use crate::stats::Stats;

mod config;
mod error;
mod stats;
mod read;
mod hash_key;

fn print_file_summary(file: &str, stats: &Stats) {
    println!("File name   : {}", file);
    println!("File stats  : {}", stats.create_summary());
}

fn print_total_summary(stats: &Stats) {
    println!("Total stats : {}", stats.create_summary())
}

pub fn run() -> Result<(), Error> {
    let config = get_config()?;
    let mut inputs_iter = config.inputs.iter();
    match inputs_iter.next() {
        None => {
            return Err(Error::from("Need to specify at least one input file."));
        }
        Some(input) => {
            let mut stats_all = read_vcf(input)?;
            print_file_summary(input, &stats_all);
            for input in inputs_iter {
                let stats_input = read_vcf(input)?;
                print_file_summary(input, &stats_input);
                stats_all.add(stats_input);
                print_total_summary(&stats_all);
            }
        }
    }
    Ok(())
}