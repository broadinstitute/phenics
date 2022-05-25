use std::io::{Read, Seek, SeekFrom, ErrorKind};
use tokio::runtime::Runtime;
use crate::error::Error;
use futures_util::StreamExt;
use futures_core;
use futures_core::Stream;
use bytes::Bytes;
use std::io;
use std::slice::Iter;

pub(crate) struct GcsReader {
    url: String,
    runtime: Runtime,
    intake: Intake
}

struct Intake {
    bytes_stream: Box<dyn Stream<Item = reqwest::Result<Bytes>>>,
    bytes: Option<Bytes>,
    i_byte: usize
}

impl GcsReader {
    pub(crate) fn connect(url: &str) -> Result<GcsReader, Error> {
        let url = String::from(url);
        GcsReader::new(url)
    }
    pub(crate) fn new(url: String) -> Result<GcsReader, Error> {
        let mut runtime = Runtime::new()?;
        let intake = Intake::open(&url, &mut runtime)?;
        Ok(GcsReader { url, runtime, intake })
    }
}

impl Intake {
    fn new(bytes_stream: Box<dyn Stream<Item = reqwest::Result<Bytes>>>, bytes: Option<Bytes>)
        -> Intake {
        let i_byte = 0usize;
        Intake { bytes_stream, bytes, i_byte }
    }
    fn open(url: &str, runtime: &Runtime) -> Result<Intake, Error>  {
        runtime.block_on(async {
            let response = reqwest::get(&*url).await?;
            let mut bytes_stream = Box::new(response.bytes_stream());
            let bytes = match bytes_stream.next().await {
                None => None,
                Some(result) => Some(result?)
            };
            Ok(Intake::new(bytes_stream, bytes))
        })
    }
}

impl Read for GcsReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let GcsReader { runtime, url, intake } = self;
        match &intake.bytes {
            None => { Ok(0usize) }
            Some(bytes) => {
                bytes.len()
            }
        }
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