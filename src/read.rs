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
    let mut stats = Stats::new(sample_ids);
    for record in reader.records(&header) {
        let record = record?;
        let genotypes = record.genotypes().genotypes()?;
        if genotypes.len() != stats.n_samples() {
            return
                Err(Error::from(
                    format!("At {}:{}, {} genotypes, but {} samples.", record.chromosome(),
                            record.position(), genotypes.len(), stats.n_samples())
                ));
        }

        stats.add_record();
    }
    Ok(stats)
}