use crate::stats::Stats;
use crate::error::Error;
use noodles::{bgzf, vcf};
use fs_err::File;
use noodles::vcf::Header;
use crate::stats::genotype_stats::GenotypeStats;
use crate::locus::Locus;

pub(crate) fn read_vcf(file: &str) -> Result<Stats, Error> {
    let mut reader =
        vcf::Reader::new(bgzf::Reader::new(File::open(file)?));
    let header = reader.read_header()?.parse::<Header>()?;
    let sample_ids: Vec<String> = header.sample_names().iter().map(String::from).collect();
    let mut stats = Stats::new(sample_ids);
    for record in reader.records(&header) {
        let record = record?;
        let genotypes = record.genotypes().genotypes()?;
        let x = record.alternate_bases();
        let locus = Locus::new(record.chromosome(), &record.position());
        stats.check_same_size_as_samples(&genotypes, &locus, "genotypes")?;
        let mut genotype_stats_list = Vec::<Option<GenotypeStats>>::new();
        for genotype in genotypes {
            let genotype_stats =
                genotype.map(|genotype|{ GenotypeStats::new(genotype) });
            genotype_stats_list.push(genotype_stats);
        }
        stats.add_genotypes(&locus, genotype_stats_list)?;
    }
    Ok(stats)
}