use crate::error::Error;
use crate::sim::Sim;
use std::io::{BufWriter, BufReader, BufRead};
use fs_err::File;
use std::io::Write;
use crate::sim::sample_sim::SampleSim;
use crate::sim;

const N_RECORDS: &str = "n_records";
const HEADER_PREFIX: &str = "#id\tn_no_gt\tn_no_alt";

pub(crate) fn write(sim: &Sim, file: &str) -> Result<(), Error> {
    let mut writer = BufWriter::new(File::create(file)?);
    writeln!(writer, "##version={}", env!("CARGO_PKG_VERSION"))?;
    writeln!(writer, "##{}={}", N_RECORDS, sim.n_records)?;
    writeln!(writer, "##n_samples={}", sim.sample_sims.len())?;
    writeln!(writer, "##n_phenotypes={}", sim.phenotype_names.len())?;
    let phenotypes = sim.phenotype_names.join("\t");
    writeln!(writer, "{}{}", HEADER_PREFIX, phenotypes)?;
    for sample in &sim.sample_sims {
        let effects =
            sample.effects.iter().map(|effect| { effect.to_string() })
                .collect::<Vec<String>>().join("\t");
        writeln!(writer, "{}\t{}\t{}\t{}", sample.id, sample.n_unknown_genotypes,
                 sample.n_unknown_alleles, effects)?;
    }
    Ok(())
}

fn missing(name: &str) -> Error {
    Error::from(format!("Missing `{}`", name))
}

fn ensure_value(value: Option<&str>) -> Result<&str, Error> {
    value.ok_or_else(|| { Error::from("Not enough values.") })
}

fn ensure_u64_at(value: Option<&str>, id: &str) -> Result<u64, Error> {
    match value {
        None => { Err(Error::from(format!("Missing value for {}.", id))) }
        Some(value) => {
            let number = value.parse::<u64>()?;
            Ok(number)
        }
    }
}

pub(crate) fn read(file: &str) -> Result<Sim, Error> {
    let lines = BufReader::new(File::open(file)?).lines();
    let mut n_records: Option<u64> = None;
    let mut phenotype_names: Option<Vec<String>> = None;
    let mut sample_sims: Vec<SampleSim> = Vec::new();
    for line in lines {
        let line = line?;
        if let Some(line) = line.strip_prefix("##") {
            if let Some(eq_pos) = line.find('=') {
                let field = &line[0..eq_pos];
                let value = &line[(eq_pos + 1)..];
                if field == N_RECORDS {
                    n_records = Some(value.parse()?)
                }
            }
        } else if let Some(line) = line.strip_prefix(HEADER_PREFIX) {
            phenotype_names = Some(line.split('\t').map(String::from).collect())
        } else if line.strip_prefix('#').is_some() {
            return Err(Error::from("Header is not in expected format."));
        } else if let Some(phenotype_names) = &phenotype_names {
            let mut values = line.split('\t');
            let id = String::from(ensure_value(values.next())?);
            let n_unknown_genotypes = ensure_u64_at(values.next(), &id)?;
            let n_unknown_alleles = ensure_u64_at(values.next(), &id)?;
            let mut effects: Vec<f64> = Vec::new();
            for value in values {
                effects.push(value.parse()?);
            }
            if phenotype_names.len() != effects.len() {
                return Err(Error::from(
                    format!("Got {} effects, but {} phenotypes.",
                            effects.len(), phenotype_names.len())
                ));
            }
            let sample_sim = SampleSim { id, effects, n_unknown_genotypes, n_unknown_alleles };
            sample_sims.push(sample_sim);
        } else {
            return Err(Error::from("Missing header line"));
        }
    }
    let n_records = n_records.ok_or_else(|| { missing(N_RECORDS) })?;
    let phenotype_names = phenotype_names.ok_or_else(|| { missing(N_RECORDS) })?;
    Ok(Sim { phenotype_names, sample_sims, n_records })
}

pub(crate) fn read_merge(inputs: &[String]) -> Result<Sim, Error> {
    let mut inputs_iter = inputs.iter();
    match inputs_iter.next() {
        None => {
            Err(Error::from("Need to specify at least one input file."))
        }
        Some(input) => {
            println!("Next reading {}", input);
            let mut sim_all = sim::io::read(input)?;
            println!("File: {}", sim_all.create_summary());
            for input in inputs_iter {
                println!("Next reading {}", input);
                let sim_input = sim::io::read(input)?;
                println!("File: {}", sim_input.create_summary());
                sim_all = sim_all.try_add(&sim_input)?;
                println!("All : {}", sim_all.create_summary());
            }
            Ok(sim_all)
        }
    }
}