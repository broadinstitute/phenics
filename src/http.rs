pub(crate) struct Range {
    from: Option<usize>,
    to: Option<usize>,
}

impl Range {
    pub(crate) fn new(from: Option<usize>, to: Option<usize>) -> Range {
        Range { from, to }
    }
    pub(crate) fn as_header(&self) -> String {
        let from = self.from.unwrap_or(0);
        let to = self.to.map(|to| { to.to_string() }).unwrap_or("".to_string());
        format!("bytes={}-{}", from, to)
    }
}