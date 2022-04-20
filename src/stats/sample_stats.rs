struct SampleStats {
    n_unknown_alleles: u64,
}

impl SampleStats {
    fn new() -> SampleStats {
        let n_unknown_alleles = 0u64;
        SampleStats { n_unknown_alleles }
    }
    fn merge_stats(&mut self, o_stats: &SampleStats) {
        self.n_unknown_alleles += o_stats.n_unknown_alleles;
    }
}