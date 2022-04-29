use clap::{command, Arg};
use crate::error::Error;
use crate::error;

pub(crate) struct Config {
    pub(crate) inputs: Vec<String>,
    pub(crate) phenotype_file: String
}

pub(crate) fn get_config() -> Result<Config, Error> {
    const INPUT: &str = "input";
    const PHENOTYPE: &str = "phenotype";
    let app = command!()
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
        );
    let matches = app.try_get_matches()?;
    let inputs =
        error::none_to_error(matches.values_of(INPUT),
                             "Need to specify input files")?
            .map(String::from).collect();
    let phenotype_file =
        String::from(error::none_to_error(matches.value_of(PHENOTYPE),
                                              "Need to specify phenotype definitions")?);
    Ok(Config { inputs, phenotype_file })
}