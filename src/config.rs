use clap::{command, Arg, Values};
use crate::error::Error;
use crate::error;
use crate::phenotype::Phenotype;

pub(crate) struct Config {
    pub(crate) inputs: Vec<String>,
    phenotypes: Vec<Phenotype>
}

fn parse_phenotypes(phenotype_args: Values) -> Result<Vec<Phenotype>, Error> {
    let mut phenotypes = Vec::<Phenotype>::new();
    for arg in phenotype_args {
        phenotypes.push(Phenotype::parse(arg)?);
    }
    Ok(phenotypes)
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
            .value_name("PHENOTYPE")
            .multiple_values(true)
            .help("Phenotype definition")
        );
    let matches = app.try_get_matches()?;
    let inputs =
        error::none_to_error(matches.values_of(INPUT),
                             "Need to specify input files")?
            .map(String::from).collect();
    let phenotypes =
        parse_phenotypes(error::none_to_error(matches.values_of(PHENOTYPE),
                                              "Need to specify phenotype definitions")?)?;
    Ok(Config { inputs, phenotypes })
}