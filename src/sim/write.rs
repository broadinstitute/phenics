use crate::error::Error;
use crate::sim::Sim;
use std::io::BufWriter;
use fs_err::File;
use std::io::Write;

pub(crate) fn write(sim: &Sim, file_name: &str) -> Result<(), Error> {
    let mut writer = BufWriter::new(File::create(file_name)?);
    writeln!(writer, "##version={}", env!("CARGO_PKG_VERSION"))?;
    writeln!(writer, "##n_records={}", sim.n_records)?;
    writeln!(writer, "##n_samples={}", sim.sample_sims.len())?;
    writeln!(writer, "##n_phenotypes={}", sim.phenotype_names.len())?;
    let phenotypes = sim.phenotype_names.join("\t");
    writeln!(writer, "#id\tn_no_gt\tn_no_alt\t{}", phenotypes)?;
    for sample in &sim.sample_sims {
        let effects =
            sample.effects.iter().map(|effect| { effect.to_string() })
                .collect::<Vec<String>>().join("\t");
        writeln!(writer, "{}\t{}\t{}\t{}", sample.id, sample.n_unknown_genotypes,
                 sample.n_unknown_alleles, effects)?;
    }
    Ok(())
}