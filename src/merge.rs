use crate::config::MergeConfig;
use crate::error::Error;
use crate::sim;

pub(crate) fn merge(config: &MergeConfig) -> Result<(), Error> {
    let sim = sim::io::read_merge(&config.inputs)?;
    sim::io::write(&sim, &config.output)?;
    Ok(())
}