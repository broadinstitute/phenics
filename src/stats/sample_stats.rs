pub(crate) struct SampleStats {
    n_unknown_alleles: u64,
}

impl SampleStats {
    pub(crate) fn new() -> SampleStats {
        let n_unknown_alleles = 0u64;
        SampleStats { n_unknown_alleles }
    }
    pub(crate) fn merge(&mut self, o_stats: &SampleStats) {
        self.n_unknown_alleles += o_stats.n_unknown_alleles;
    }
}