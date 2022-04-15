use crate::stats::Stats;
use crate::error::Error;
use noodles::{bgzf, vcf};
use fs_err::File;
use noodles::vcf::Header;

pub(crate) fn read_vcf(file: &str) -> Result<Stats, Error> {
    let mut reader =
        vcf::Reader::new(bgzf::Reader::new(File::open(file)?));
    let header = reader.read_header()?.parse::<Header>()?;
    let sample_ids: Vec<String> = header.sample_names().iter().map(String::from).collect();
    Ok(Stats::new(sample_ids))
}