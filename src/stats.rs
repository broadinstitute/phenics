struct Stats {
    sample_ids: Vec<String>
}

impl Stats {
    fn new(sample_ids: Vec<String>) -> Stats {
        Stats { sample_ids }
    }
}