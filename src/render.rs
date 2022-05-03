pub(crate) mod pheno_result;
pub(crate) mod sample_result;

use crate::config::RenderConfig;
use crate::error::Error;
use crate::{sim, phenotype};

pub(crate) fn render(config: &RenderConfig) -> Result<(), Error> {
    let sim = sim::io::read_merge(&config.inputs)?;
    let phenotypes = phenotype::load::load(&config.phenotype_file)?;
    sim.render_phenotypes(&phenotypes)?;
    todo!()
}