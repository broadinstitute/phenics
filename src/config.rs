use clap::{command, Arg, Command};
use crate::error::Error;
use crate::error;

pub(crate) enum Config {
    Vcf(VcfConfig)
}

pub(crate) struct VcfConfig {
    pub(crate) input: Vec<String>,
    pub(crate) phenotype_file: String,
}

pub(crate) fn get_config() -> Result<Config, Error> {
    const VCF: &str = "vcf";
    const INPUT: &str = "input";
    const PHENOTYPE: &str = "phenotype";
    let app = command!()
        .subcommand(
            Command::new(VCF)
                .arg_required_else_help(true)
                .arg(Arg::new(INPUT)
                    .short('i')
                    .long(INPUT)
                    .takes_value(true)
                    .value_name("FILE")
                    .multiple_values(true)
                    .help("Input files (VCF)")
                )
                .arg(Arg::new(PHENOTYPE)
                    .short('p')
                    .long(PHENOTYPE)
                    .takes_value(true)
                    .value_name("FILE")
                    .help("Phenotype definitions file")
                )
        );
    let arg_matches = app.try_get_matches()?;
    match arg_matches.subcommand() {
        Some((VCF, vcf_matches)) => {
            let inputs =
                error::none_to_error(vcf_matches.values_of(INPUT),
                                     "Need to specify input files")?
                    .map(String::from).collect();
            let phenotype_file =
                String::from(error::none_to_error(vcf_matches.value_of(PHENOTYPE),
                                                  "Need to specify phenotype definitions")?);
            Ok(Config::Vcf(VcfConfig { input: inputs, phenotype_file }))
        }
        Some(match_with_sub) => {
            let subcommand_name = match_with_sub.0;
            Err(Error::from(format!("Unknown subcommand {}. Available is only {}.",
                                    subcommand_name, VCF)))
        }
        None => {
            Err(Error::from(format!("Missing. Available is only {}.", VCF)))
        }
    }
}