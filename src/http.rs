use reqwest::Response;
use crate::error::Error;

pub(crate) struct Range {
    pub(crate) from: Option<u64>,
    pub(crate) to: Option<u64>,
}

impl Range {
    pub(crate) fn new(from: Option<u64>, to: Option<u64>) -> Range {
        Range { from, to }
    }
    pub(crate) fn new_from(from: u64) -> Range { Range::new(Some(from), None) }
    pub(crate) fn as_header(&self) -> String {
        let from = self.from.unwrap_or(0);
        let to = self.to.map(|to| { to.to_string() }).unwrap_or_else(|| "".to_string());
        format!("bytes={}-{}", from, to)
    }
}

pub(crate) fn parse_size(response: &Response) -> Result<Option<u64>, Error> {
    match response.headers().get("Content-Range") {
        None => { Ok(response.content_length()) }
        Some(value) => {
            let value_str = value.to_str()?;
            let mut parts = value_str.split('/');
            let size = parts.nth(1).ok_or_else(|| {
                Error::from(
                    format!("Could not parse 'Content-Range header value '{}'.", value_str)
                )
            })?.parse::<u64>()?;
            Ok(Some(size))
        }
    }
}