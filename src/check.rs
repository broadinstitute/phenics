use crate::config::CheckConfig;
use crate::error::Error;
use crate::phenotype::load::load;

pub(crate) fn check(config: &CheckConfig) -> Result<(), Error> {
    let phenotypes = load(&config.phenotype_file)?;
    for phenotype in phenotypes {
        println!("{}={}", phenotype.name, phenotype.sim);
    }
    Ok(())
}