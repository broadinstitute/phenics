use std::io::{Read, Seek, SeekFrom};
use tokio::runtime::Runtime;
use crate::error::Error;
use futures_util::StreamExt;
use futures_core::Stream;
use bytes::{Bytes, Buf};
use std::pin::Pin;
use std::io;
use reqwest::RequestBuilder;
use crate::http::Range;
use crate::http;

pub(crate) struct GcsReader {
    url: String,
    runtime: Runtime,
    intake: Intake,
}

struct Intake {
    bytes_stream: Pin<Box<dyn Stream<Item=reqwest::Result<Bytes>>>>,
    bytes: Option<Bytes>,
    pos: u64,
    size: Option<u64>
}

impl GcsReader {
    pub(crate) fn connect(url: &str, range: &Range) -> Result<GcsReader, Error> {
        let url = String::from(url);
        GcsReader::new(url, range)
    }
    pub(crate) fn new(url: String, range: &Range) -> Result<GcsReader, Error> {
        let runtime = Runtime::new()?;
        let intake = Intake::open(&url, &runtime, range)?;
        Ok(GcsReader { url, runtime, intake })
    }
}

impl Intake {
    fn new(bytes_stream: Pin<Box<dyn Stream<Item=reqwest::Result<Bytes>>>>, bytes: Option<Bytes>,
           pos: u64, size: Option<u64>)
           -> Intake {
        Intake { bytes_stream, bytes, pos, size }
    }
    fn open(url: &str, runtime: &Runtime, range: &Range) -> Result<Intake, Error> {
        runtime.block_on(async {
            let response = Intake::build_request(url, range).send().await?;
            let mut bytes_stream = Box::pin(response.bytes_stream());
            let bytes = match bytes_stream.next().await {
                None => None,
                Some(result) => Some(result?)
            };
            let pos = range.from.unwrap_or(0);
            let size = http::parse_size(&response)?;
            Ok(Intake::new(bytes_stream, bytes, pos, size))
        })
    }
    fn build_request(url: &str, range: &Range) -> RequestBuilder {
        reqwest::Client::new().get(&*url).header("Range", range.as_header())
    }
}

impl Read for GcsReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let GcsReader { runtime, intake, .. } = self;
        let need_next_bytes =
            if let Some(bytes) = &intake.bytes {
                bytes.is_empty()
            } else {
                false
            };
        if need_next_bytes {
            let bytes = runtime.block_on(async {
                let bytes_stream = &mut intake.bytes_stream;
                let bytes = match bytes_stream.next().await {
                    None => None,
                    Some(result) => {
                        Some(result.map_err(|reqwest_error| {
                            io::Error::new(io::ErrorKind::Other, reqwest_error)
                        })?)
                    }
                };
                Ok::<Option<Bytes>, io::Error>(bytes)
            })?;
            intake.bytes = bytes;
        }
        match &mut intake.bytes {
            None => { Ok(0usize) }
            Some(bytes) => {
                let n_bytes = std::cmp::min(buf.len(), bytes.len());
                bytes.split_to(n_bytes).copy_to_slice(&mut buf[0..n_bytes]);
                intake.pos += n_bytes as u64;
                Ok(n_bytes)
            }
        }
    }
}

impl Seek for GcsReader {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Start(pos) => {
                self.intake =
                    Intake::open(&self.url, &self.runtime, &Range::new_from(pos))?;
            }
            SeekFrom::End(_) => {}
            SeekFrom::Current(_) => {}
        }
        todo!()
    }
}
