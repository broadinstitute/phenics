use std::ops::RangeBounds;
use std::time::{Duration, Instant};
use noodles::{bgzf, vcf, core};
use crate::config::GcsTabixConfig;
use crate::Error;
use noodles::tabix::{Index, Reader};
use crate::gcs::GcsReader;
use noodles::csi::binning_index::BinningIndex;
use crate::http::Range;
use crate::records::{RecordPrinter, RecordProcessor};
use crate::region_iter::RegionIterGen;

pub(crate) fn tabix(config: &GcsTabixConfig) -> Result<(), Error> {
    let mut record_processor = RecordPrinter::new();
    process_region(&config.data, &config.index, &config.region,
                   &mut record_processor)?;
    Ok(())
}

pub(crate) fn process_region<P: RecordProcessor>(data_url: &str, index_url: &str,
                                                 region: &core::Region, record_processor: &mut P)
                                                 -> Result<usize, Error> {
    let index = read_index(index_url)?;
    let vcf_header = read_vcf_header(data_url)?;
    let n_records =
        if let Some((i_chrom, _)) = index.reference_sequence_names().get_full(region.name()) {
            read_region(&vcf_header, &index, data_url, i_chrom, region, record_processor)?
        } else {
            0
        };
    Ok(n_records)
}

pub(crate) fn sample_regions<P: RecordProcessor>(data_url: &str, index_url: &str,
                                                 vcf_header: &vcf::Header, record_processor: &mut P,
                                                 region_iter_gen: &RegionIterGen)
                                                 -> Result<usize, Error> {
    let index = read_index(index_url)?;
    let mut n_records: usize = 0;
    for (i_chrom, chrom) in index.reference_sequence_names().iter().enumerate() {
        let region_iter = region_iter_gen.new_region_iter(chrom.clone());
        for region in region_iter {
            println!("Now reading region {}.", region);
            let n_records_new =
                read_region(vcf_header, &index, data_url, i_chrom, &region, record_processor)?;
            println!("Read {} records from region {}.", n_records_new, region);
            n_records += n_records_new;
        }
    }
    Ok(n_records)
}

fn read_index(index_url: &str) -> Result<Index, Error> {
    let mut index_reader = Reader::new(GcsReader::connect(index_url)?);
    let index = index_reader.read_index()?;
    Ok(index)
}

pub(crate) fn read_vcf_header(data_url: &str) -> Result<vcf::Header, Error> {
    let mut data_reader =
        vcf::Reader::new(bgzf::Reader::new(GcsReader::connect(data_url)?));
    let vcf_header = data_reader.read_header()?.parse::<vcf::Header>()?;
    Ok(vcf_header)
}

struct StatusReporter {
    start: Instant,
    duration_old: Duration,
    n_records_old: usize,
    n_records: usize,
}

impl StatusReporter {
    fn new() -> StatusReporter {
        let start = Instant::now();
        let duration_previously = start.elapsed();
        let n_records_old = 0;
        let n_records = 0;
        StatusReporter { start, duration_old: duration_previously, n_records_old, n_records }
    }
    fn count_record(&mut self) {
        self.n_records += 1
    }
    fn report(&mut self) {
        let duration = self.start.elapsed();
        println!("After {} seconds, read {} records", duration.as_secs(), self.n_records);
        self.duration_old = duration;
        self.n_records_old = self.n_records;
    }
    fn report_maybe(&mut self) {
        let duration = self.start.elapsed();
        if duration.as_secs() > self.duration_old.as_secs() + 10 ||
            self.n_records > self.n_records_old + 100 {
            self.report();
        }
    }
}

fn read_region<P: RecordProcessor>(vcf_header: &vcf::Header, index: &Index, data_url: &str,
                                   i_chrom: usize, region: &core::Region, record_processor: &mut P)
                                   -> Result<usize, Error> {
    let chunks = index.query(i_chrom, region.interval())?;
    let mut n_records: usize = 0;
    println!("Got {} chunks for {}", chunks.len(), region);
    for chunk in chunks {
        let range =
            Range::new(Some(chunk.start().compressed()),
                       Some(chunk.end().compressed() - 1));
        let mut bgzf_reader =
            bgzf::Reader::new(GcsReader::connect_range(data_url, &range)?);
        bgzf_reader.seek(chunk.start())?;
        let mut vcf_reader = vcf::Reader::new(bgzf_reader);
        let mut status_reporter = StatusReporter::new();
        for record in vcf_reader.records(vcf_header) {
            let record = record?;
            let record_position =
                core::Position::try_from(usize::from(record.position()))?;
            if region.interval().contains(&record_position) {
                n_records += 1;
                status_reporter.count_record();
                record_processor.process_record(&record)?;
            } else if let Some(end) = region.interval().end() {
                if record_position > end {
                    println!("Record position {} after of region {} - break.",
                             record_position, region.interval());
                    break;
                }
            }
            status_reporter.report_maybe();
        }
        println!("Done with chunk");
        status_reporter.report();
    }
    Ok(n_records)
}

