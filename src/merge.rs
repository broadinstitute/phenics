use crate::config::MergeConfig;
use crate::error::Error;
use crate::sim;

pub(crate) fn merge(config: &MergeConfig) -> Result<(), Error> {
    let mut inputs_iter = config.inputs.iter();
    match inputs_iter.next() {
        None => {
            return Err(Error::from("Need to specify at least one input file."));
        }
        Some(input) => {
            println!("Next reading {}", input);
            let mut sim_all = sim::io::read(input)?;
            println!("File: {}", sim_all.create_summary());
            for input in inputs_iter {
                println!("Next reading {}", input);
                let sim_input = sim::io::read(input)?;
                println!("File: {}", sim_input.create_summary());
                sim_all = sim_all.try_add(&sim_input)?;
                println!("All : {}", sim_all.create_summary());
            }
            sim::io::write(&sim_all, &config.output)?;
        }
    }
    Ok(())
}