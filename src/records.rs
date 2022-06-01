use noodles::vcf::Record;
use crate::error::Error;
use crate::sim::genotype_sim::GenotypeSim;
use crate::sim::allele_sim::AlleleSim;
use crate::locus::Locus;
use crate::sim::Sim;
use crate::phenotype::Phenotype;

pub(crate) trait RecordProcessor {
    fn process_record(&mut self, record: &Record) -> Result<(), Error>;
}

pub(crate) struct SimProcessor<'a> {
    phenotypes: &'a [Phenotype],
    sim: &'a mut Sim,
}

pub(crate) struct RecordPrinter {}

impl SimProcessor<'_> {
    pub(crate) fn new<'a>(sim: &'a mut Sim, phenotypes: &'a [Phenotype]) -> SimProcessor<'a> {
        SimProcessor { sim, phenotypes }
    }
}

impl RecordProcessor for SimProcessor<'_> {
    fn process_record(&mut self, record: &Record) -> Result<(), Error> {
        let genotypes = record.genotypes().genotypes()?;
        let allele_sims =
            record.alternate_bases().iter().map(|_alts| {
                AlleleSim::from_phenotypes(self.phenotypes)
            })
                .collect::<Vec<AlleleSim>>();
        let locus = Locus::new(record.chromosome(), &record.position());
        self.sim.check_same_size_as_samples(&genotypes, &locus, "genotypes")?;
        for (i_sample, genotype) in genotypes.iter().enumerate() {
            let genotype_sim =
                genotype.as_ref()
                    .map(|genotype| { GenotypeSim::new(genotype, allele_sims.len()) });
            self.sim.add_genotype_sim(&genotype_sim, i_sample, &allele_sims);
        }
        self.sim.count_record();
        Ok(())
    }
}

impl RecordProcessor for RecordPrinter {
    fn process_record(&mut self, record: &Record) -> Result<(), Error> {
        println!("{}", record);
        Ok(())
    }
}