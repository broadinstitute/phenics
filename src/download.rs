use crate::config::DownloadConfig;
use crate::error::Error;
use fs_err::File;
use std::io::{BufWriter, BufReader, Read, Write};

pub(crate) fn download(config: &DownloadConfig) -> Result<(), Error> {
    let mut reader = BufReader::new(File::open("dummy")?);
    let mut writer = BufWriter::new(File::create(&config.output)?);
    const BUFFER_SIZE: usize = 1028;
    let mut buffer = [0u8; BUFFER_SIZE];
    loop {
        let n_read = reader.read(&mut buffer)?;
        if n_read == 0 {
            break
        }
        writer.write_all(&buffer[..n_read])?;
    }
    Ok(())
}