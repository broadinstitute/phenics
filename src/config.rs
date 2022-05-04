use clap::{command, Arg, Command};
use crate::error::Error;
use crate::error;

pub(crate) enum Config {
    Check(CheckConfig),
    Vcf(VcfConfig),
    Merge(MergeConfig),
    Render(RenderConfig),
}

pub(crate) struct CheckConfig {
    pub(crate) phenotype_file: String,
}

pub(crate) struct VcfConfig {
    pub(crate) inputs: Option<Vec<String>>,
    pub(crate) phenotype_file: String,
    pub(crate) output: String,
}

pub(crate) struct MergeConfig {
    pub(crate) inputs: Vec<String>,
    pub(crate) output: String,
}

pub(crate) struct RenderConfig {
    pub(crate) inputs: Vec<String>,
    pub(crate) phenotype_file: String,
    pub(crate) output: String,
}

const CHECK: &str = "check";
const VCF: &str = "vcf";
const MERGE: &str = "merge";
const RENDER: &str = "render";
const INPUT: &str = "input";
const OUTPUT: &str = "output";
const PHENOTYPE: &str = "phenotype";

fn subcommand_problem(problem: &str) -> Result<Config, Error> {
    let message =
        format!("{}. Available are '{}', '{}', '{}' and '{}'.",
                problem, CHECK, VCF, MERGE, RENDER);
    Err(Error::from(message))
}

pub(crate) fn get_config() -> Result<Config, Error> {
    let app = command!()
        .subcommand(
            Command::new(CHECK)
                .arg_required_else_help(true)
                .arg(Arg::new(PHENOTYPE)
                    .short('p')
                    .long(PHENOTYPE)
                    .takes_value(true)
                    .value_name("FILE")
                    .help("Phenotype definitions file")
                )
        )
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
                .arg(Arg::new(OUTPUT)
                    .short('o')
                    .long(OUTPUT)
                    .takes_value(true)
                    .value_name("FILE")
                    .help("Output file")
                )
        )
        .subcommand(
            Command::new(MERGE)
                .arg_required_else_help(true)
                .arg(Arg::new(INPUT)
                    .short('i')
                    .long(INPUT)
                    .takes_value(true)
                    .value_name("FILE")
                    .multiple_values(true)
                    .help("Input files (liabilities)")
                )
                .arg(Arg::new(OUTPUT)
                    .short('o')
                    .long(OUTPUT)
                    .takes_value(true)
                    .value_name("FILE")
                    .help("Output file")
                )
        ).subcommand(
        Command::new(RENDER)
            .arg_required_else_help(true)
            .arg(Arg::new(INPUT)
                .short('i')
                .long(INPUT)
                .takes_value(true)
                .value_name("FILE")
                .multiple_values(true)
                .help("Input files (liabilities)")
            )
            .arg(Arg::new(PHENOTYPE)
                .short('p')
                .long(PHENOTYPE)
                .takes_value(true)
                .value_name("FILE")
                .help("Phenotype definitions file")
            )
            .arg(Arg::new(OUTPUT)
                .short('o')
                .long(OUTPUT)
                .takes_value(true)
                .value_name("FILE")
                .help("Output file")
            )
    )
        ;
    let arg_matches = app.try_get_matches()?;
    match arg_matches.subcommand() {
        Some((CHECK, check_matches)) => {
            let phenotype_file =
                String::from(
                    error::none_to_error(check_matches.value_of(PHENOTYPE),
                                         "Need to specify phenotype definitions")?);
            Ok(Config::Check(CheckConfig { phenotype_file }))
        }
        Some((VCF, vcf_matches)) => {
            let inputs = vcf_matches.values_of(INPUT).map(|values| {
                values.map(|value| { String::from(value) }).collect()
            });
            let phenotype_file =
                String::from(
                    error::none_to_error(vcf_matches.value_of(PHENOTYPE),
                                         "Need to specify phenotype definitions")?);
            let output =
                String::from(error::none_to_error(vcf_matches.value_of(OUTPUT),
                                                  "Need to specify output file.")?);
            Ok(Config::Vcf(VcfConfig { inputs, phenotype_file, output }))
        }
        Some((MERGE, merge_matches)) => {
            let inputs =
                error::none_to_error(merge_matches.values_of(INPUT),
                                     "Need to specify input files")?
                    .map(String::from).collect();
            let output =
                String::from(error::none_to_error(merge_matches.value_of(OUTPUT),
                                                  "Need to specify output file.")?);
            Ok(Config::Merge(MergeConfig { inputs, output }))
        }
        Some((RENDER, render_matches)) => {
            let inputs =
                error::none_to_error(render_matches.values_of(INPUT),
                                     "Need to specify input files")?
                    .map(String::from).collect();
            let phenotype_file =
                String::from(
                    error::none_to_error(render_matches.value_of(PHENOTYPE),
                                         "Need to specify phenotype definitions")?);
            let output =
                String::from(error::none_to_error(render_matches.value_of(OUTPUT),
                                                  "Need to specify output file.")?);
            Ok(Config::Render(RenderConfig { inputs, phenotype_file, output }))
        }
        Some(match_with_sub) => {
            let subcommand_name = match_with_sub.0;
            subcommand_problem(&format!("Unknown subcommand {}.", subcommand_name))
        }
        None => {
            subcommand_problem("Missing subcommand")
        }
    }
}