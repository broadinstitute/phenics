use crate::render::pheno_result::PhenoResult;

pub(crate) struct SampleResult<'a> {
    name: String,
    pheno_results: Vec<PhenoResult<'a>>
}