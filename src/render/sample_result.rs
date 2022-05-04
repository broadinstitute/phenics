use crate::render::pheno_result::PhenoResult;

pub(crate) struct SampleResult {
    pub(crate) name: String,
    pub(crate) pheno_results: Vec<PhenoResult>
}

impl SampleResult {
    pub(crate) fn new(name: String, pheno_results: Vec<PhenoResult>) -> SampleResult {
        SampleResult { name, pheno_results }
    }
}