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
use crate::gc_auth::GCAuth;

pub(crate) struct GcsReader {
    url: String,
    runtime: Runtime,
    gc_auth: GCAuth,
    intake: Intake,
}

struct Intake {
    bytes_stream: Pin<Box<dyn Stream<Item=reqwest::Result<Bytes>>>>,
    bytes: Option<Bytes>,
    pos: u64,
    size: Option<u64>,
}

impl GcsReader {
    pub(crate)  fn get_url(url_raw: &str) -> String {
        if let Some(path) = url_raw.strip_prefix("gs://") {
            format!("https://storage.cloud.google.com/{}", path)
        } else {
            String::from(url_raw)
        }
    }
    pub(crate) fn connect(url: &str) -> Result<GcsReader, Error> {
        let url = GcsReader::get_url(url);
        let range = Range::new_from(0);
        GcsReader::new(url, &range)
    }
    pub(crate) fn connect_range(url: &str, range: &Range) -> Result<GcsReader, Error> {
        let url = GcsReader::get_url(url);
        GcsReader::new(url, range)
    }
    pub(crate) fn new(url: String, range: &Range) -> Result<GcsReader, Error> {
        let runtime = Runtime::new()?;
        let gc_auth = runtime.block_on(async { GCAuth::new().await })?;
        let intake = Intake::open(&url, &runtime, range, &gc_auth)?;
        Ok(GcsReader { url, runtime, intake, gc_auth })
    }
    fn seek_pos(&mut self, pos: u64) -> std::io::Result<()> {
        self.intake =
            Intake::open(&self.url, &self.runtime, &Range::new_from(pos), &self.gc_auth)
                .map_err(|error| { error.into_io_error() })?;
        Ok(())
    }
}

impl Intake {
    fn new(bytes_stream: Pin<Box<dyn Stream<Item=reqwest::Result<Bytes>>>>, bytes: Option<Bytes>,
           pos: u64, size: Option<u64>)
           -> Intake {
        Intake { bytes_stream, bytes, pos, size }
    }
    fn open(url: &str, runtime: &Runtime, range: &Range, gc_auth: &GCAuth)
            -> Result<Intake, Error> {
        runtime.block_on(async {
            let token = gc_auth.get_token().await?;
            let request =
                http::add_bearer_auth(Intake::build_request(url, range), &token);
            println!("===\n{:?}\n===", request);
            let response = request.send().await?;
            let status_code = response.status();
            if !status_code.is_success() {
                match response.text().await {
                    Ok(text) => { println!("===\n{}\n===", text)}
                    Err(error) => { println!("===\n{}\n===", error)}
                }
                return Err(Error::from(format!("{} ({})", status_code, url)));
            }
            let size = http::parse_size(&response)?;
            let mut bytes_stream = Box::pin(response.bytes_stream());
            let bytes = match bytes_stream.next().await {
                None => None,
                Some(result) => Some(result?)
            };
            let pos = range.from.unwrap_or(0);
            Ok(Intake::new(bytes_stream, bytes, pos, size))
        })
    }
    fn build_request(url: &str, range: &Range) -> RequestBuilder {
        reqwest::Client::new().get(&*url).header("Range", range.as_header())
    }
}

impl Read for GcsReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        println!("read()");
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
            None => { println!("No more bytes"); Ok(0usize) }
            Some(bytes) => {
                let n_bytes = std::cmp::min(buf.len(), bytes.len());
                bytes.split_to(n_bytes).copy_to_slice(&mut buf[0..n_bytes]);
                intake.pos += n_bytes as u64;
                println!("{} bytes", n_bytes);
                Ok(n_bytes)
            }
        }
    }
}

fn add_u64_i64(x: u64, y: i64) -> u64 {
    if y == 0 {
        x
    } else if x > 0 {
        x + (y as u64)
    } else {
        x - ((-y) as u64)
    }
}

impl Seek for GcsReader {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        println!("Seek {:?}", pos);
        let pos = match pos {
            SeekFrom::Start(pos) => { pos }
            SeekFrom::End(pos_end) => {
                let size = self.intake.size.ok_or_else(|| {
                    Error::from("Cannot seek from end, because size is not known")
                }).map_err(|error| { error.into_io_error() })?;
                add_u64_i64(size, pos_end)
            }
            SeekFrom::Current(pos_rel) => { add_u64_i64(self.intake.pos, pos_rel) }
        };
        self.seek_pos(pos)?;
        println!("Done seeking.");
        Ok(pos)
    }
}
