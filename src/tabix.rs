use std::ops::RangeBounds;
use noodles::{bgzf, vcf, core};
use crate::config::GcsTabixConfig;
use crate::Error;
use noodles::tabix::{Index, Reader};
use crate::gcs::GcsReader;
use noodles::csi::binning_index::BinningIndex;
use crate::http::Range;
use crate::phenotype::Phenotype;
use crate::records::{RecordPrinter, RecordProcessor, SimProcessor};
use crate::sim::Sim;

pub(crate) fn tabix(config: &GcsTabixConfig) -> Result<(), Error> {
    let record_processor = Box::new(RecordPrinter{});
    process_region(&config.data, &config.index, &config.region, record_processor)
}

pub(crate) fn process_region(data_url: &str, index_url: &str, region: &core::Region,
                             record_processor: Box<dyn RecordProcessor>)
                             -> Result<(), Error> {
    let index = read_index(index_url)?;
    let vcf_header = read_vcf_header(data_url)?;
    if let Some((i_chrom, _)) = index.reference_sequence_names().get_full(region.name()) {
        read_region(&vcf_header, &index, data_url, i_chrom, region, record_processor)?;
    }
    Ok(())
}

pub(crate) fn sample_regions(data_url: &str, index_url: &str, sim: &mut Sim,
                             phenotypes: &[Phenotype]) -> Result<(), Error> {
    let index = read_index(index_url)?;
    let vcf_header = read_vcf_header(data_url)?;
    let record_processor = SimProcessor::new(sim, phenotypes);
    for i_chrom in 0..index.reference_sequence_names().len() {
        todo!()
    }
    Ok(())
}

fn read_index(index_url: &str) -> Result<Index, Error> {
    let mut index_reader = Reader::new(GcsReader::connect(index_url)?);
    let index = index_reader.read_index()?;
    Ok(index)
}

fn read_vcf_header(data_url: &str) -> Result<vcf::Header, Error> {
    let mut data_reader =
        vcf::Reader::new(bgzf::Reader::new(GcsReader::connect(data_url)?));
    let vcf_header = data_reader.read_header()?.parse::<vcf::Header>()?;
    Ok(vcf_header)
}

fn read_region(vcf_header: &vcf::Header, index: &Index, data_url: &str, i_chrom: usize,
               region: &core::Region, mut record_processor: Box<dyn RecordProcessor>)
    -> Result<(), Error> {
    let chunks = index.query(i_chrom, region.interval())?;
    for chunk in chunks {
        let range =
            Range::new(Some(chunk.start().compressed()), Some(chunk.end().compressed()));
        let mut bgzf_reader =
            bgzf::Reader::new(GcsReader::connect_range(data_url, &range)?);
        bgzf_reader.seek(chunk.start())?;
        let mut vcf_reader = vcf::Reader::new(bgzf_reader);
        for record in vcf_reader.records(vcf_header) {
            let record = record?;
            let record_position =
                usize::try_from(i32::from(record.position()))
                    .and_then(core::Position::try_from)?;
            if region.interval().contains(&record_position) {
                record_processor.process_record(&record)?;
            }
        }
    }
    Ok(())
}

