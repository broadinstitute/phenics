use std::io::{Read, Seek, SeekFrom, ErrorKind};
use tokio::runtime::Runtime;
use crate::error::Error;
use futures_util::StreamExt;

pub(crate) struct GcsReader {
    url: String,
    runtime: Runtime
}

impl GcsReader {
    pub(crate) fn connect(url: &str) -> Result<GcsReader, Error> {
        let url = String::from(url);
        GcsReader::new(url)
    }
    pub(crate) fn new(url: String) -> Result<GcsReader, Error> {
        let runtime = Runtime::new()?;
        Ok(GcsReader { url, runtime })
    }
}

impl Read for GcsReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let GcsReader { runtime, url } = self;
        runtime.block_on(async {
            let response = reqwest::get(&*url).await.map_err(reqwest_to_io_error)?;
            let mut bytes_stream = response.bytes_stream();
            let bytes = bytes_stream.next().await;
            todo!()
        })
    }
}

impl Seek for GcsReader {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        todo!()
    }
}

fn reqwest_to_io_error(request_error: reqwest::Error) -> std::io::Error {
    std::io::Error::new(ErrorKind::Other, request_error)
}