use std::cmp::Ordering;
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
use urlencoding::encode;

pub(crate) struct GcsObject {
    bucket: String,
    object: String,
}

pub(crate) struct GcsReader {
    url: String,
    runtime: Runtime,
    gc_auth: GCAuth,
    to: Option<u64>,
    intake: Intake,
}

struct Intake {
    bytes_stream: Pin<Box<dyn Stream<Item=reqwest::Result<Bytes>>>>,
    bytes: Option<Bytes>,
    pos: u64,
    size: Option<u64>,
}

fn url_parse_error(url: &str) -> Error {
    Error::from(format!("Cannot parse `{}` as a GCS URL.", url))
}

impl GcsObject {
    pub(crate) fn parse_url(url: &str) -> Result<GcsObject, Error> {
        let stripped =
            url.strip_prefix("gs://").ok_or_else(|| url_parse_error(url))?;
        let (bucket, object) =
            stripped.split_once('/').ok_or_else(|| url_parse_error(url))?;
        let bucket = bucket.to_string();
        let object = object.to_string();
        Ok(GcsObject { bucket, object })
    }
    pub(crate) fn as_api_url(&self) -> String {
        format!("https://storage.googleapis.com/storage/v1/b/{}/o/{}?alt=media",
                encode(&self.bucket), encode(&self.object))
    }
}

impl GcsReader {
    pub(crate) fn get_url(url_raw: &str) -> Result<String, Error> {
        if url_raw.starts_with("gs://") {
            Ok(GcsObject::parse_url(url_raw)?.as_api_url())
        } else {
            Ok(String::from(url_raw))
        }
    }
    pub(crate) fn connect(url: &str) -> Result<GcsReader, Error> {
        let url = GcsReader::get_url(url)?;
        let range = Range::new_from(0);
        GcsReader::new(url, &range)
    }
    pub(crate) fn connect_range(url: &str, range: &Range) -> Result<GcsReader, Error> {
        let url = GcsReader::get_url(url)?;
        GcsReader::new(url, range)
    }
    pub(crate) fn new(url: String, range: &Range) -> Result<GcsReader, Error> {
        let runtime = Runtime::new()?;
        let gc_auth = runtime.block_on(async { GCAuth::new().await })?;
        let intake = Intake::open(&url, &runtime, range, &gc_auth)?;
        let to = range.to;
        Ok(GcsReader { url, runtime, intake, gc_auth, to })
    }
    fn seek_pos(&mut self, pos: u64) -> std::io::Result<()> {
        self.intake =
            Intake::open(&self.url, &self.runtime, &Range::new(Some(pos), self.to),
                         &self.gc_auth)
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
            println!("=== begin request ===\n{:?}\n=== end request===", request);
            let response = request.send().await?;
            println!("=== begin response meta ===");
            println!("Status: {}", response.status());
            println!("content-length: {:?}", response.content_length());
            println!("{:?}", response.headers());
            println!("=== end response meta ===");
            let status_code = response.status();
            if !status_code.is_success() {
                match response.text().await {
                    Ok(text) => { println!("===\n{}\n===", text) }
                    Err(error) => { println!("===\n{}\n===", error) }
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
        let builder_base = reqwest::Client::new().get(&*url);
        if !range.is_everything() {
            builder_base.header("Range", range.as_header())
        } else {
            builder_base
        }
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
                if bytes.is_empty() {
                    Ok(0)
                } else {
                    let len_bytes = bytes.len();
                    let n_bytes = std::cmp::min(buf.len(), len_bytes);
                    let mut bytes_to_read = bytes.split_to(n_bytes);
                    bytes_to_read.copy_to_slice(&mut buf[0..n_bytes]);
                    intake.pos += n_bytes as u64;
                    Ok(n_bytes)
                }
            }
        }
    }
}

fn add_u64_i64(x: u64, y: i64) -> u64 {
    match y.cmp(&0_i64) {
        Ordering::Less => { x - ((-y) as u64) }
        Ordering::Equal => { x }
        Ordering::Greater => { x + (y as u64) }
    }
}

impl Seek for GcsReader {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
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
        Ok(pos)
    }
}
