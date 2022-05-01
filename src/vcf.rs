use crate::config::VcfConfig;
use crate::error::Error;
use crate::{phenotype, sim};
use crate::read::read_vcf;

pub(crate) fn process_vcf(vcf_config: &VcfConfig) -> Result<(), Error> {
    let mut inputs_iter = vcf_config.inputs.iter();
    match inputs_iter.next() {
        None => {
            return Err(Error::from("Need to specify at least one input file."));
        }
        Some(input) => {
            println!("Loading phenotypes from {}", vcf_config.phenotype_file);
            let phenotypes =
                phenotype::load::load(&vcf_config.phenotype_file)?;
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
            sim::write::write(&sim_all, &vcf_config.output)?;
        }
    }
    Ok(())
}
