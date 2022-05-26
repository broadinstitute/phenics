use clap::{command, Arg, Command};
use crate::error::Error;
use crate::error;
use std::str::FromStr;
use std::num::ParseIntError;
use crate::http::Range;

pub(crate) enum Config {
    Check(CheckConfig),
    Vcf(VcfConfig),
    Merge(MergeConfig),
    Render(RenderConfig),
    Download(DownloadConfig),
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

pub(crate) struct DownloadConfig {
    pub(crate) url: String,
    pub(crate) range: Range,
    pub(crate) output: String,
}

const CHECK: &str = "check";
const VCF: &str = "vcf";
const MERGE: &str = "merge";
const RENDER: &str = "render";
const DOWNLOAD: &str = "download";
const INPUT: &str = "input";
const OUTPUT: &str = "output";
const PHENOTYPE: &str = "phenotype";
const URL: &str = "url";
const FROM: &str = "from";
const TO: &str = "to";

fn subcommand_problem(problem: &str) -> Result<Config, Error> {
    let message =
        format!("{}. Available are '{}', '{}', '{}', '{}' and '{}'.",
                problem, CHECK, VCF, MERGE, RENDER, DOWNLOAD);
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
    ).subcommand(
        Command::new(DOWNLOAD)
            .arg_required_else_help(true)
            .arg(Arg::new(URL)
                .short('u')
                .long(URL)
                .takes_value(true)
                .value_name("URL")
                .help("URL to download")
            )
            .arg(Arg::new(FROM)
                .short('f')
                .long(FROM)
                .takes_value(true)
                .value_name("POS")
                .help("Start position in the object to download.")
            )
            .arg(Arg::new(TO)
                .short('t')
                .long(TO)
                .takes_value(true)
                .value_name("POS")
                .help("End position in the object to download.")
            )
            .arg(Arg::new(OUTPUT)
                .short('o')
                .long(OUTPUT)
                .takes_value(true)
                .value_name("FILE")
                .help("Output file")
            )
    );
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
        Some((DOWNLOAD, download_matches)) => {
            let url =
                String::from(error::none_to_error(download_matches.value_of(URL),
                                     "Need to specify input files")?);
            let from =
                parse_unpack::<usize, ParseIntError>(download_matches.value_of(FROM))?;
            let to =
                parse_unpack::<usize, ParseIntError>(download_matches.value_of(TO))?;
            let output =
                String::from(error::none_to_error(download_matches.value_of(OUTPUT),
                                                  "Need to specify output file.")?);
            let range = Range::new(from, to);
            Ok(Config::Download(DownloadConfig { url, range, output }))
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

fn parse_unpack<T: FromStr, E: From<<T as FromStr>::Err>>(text: Option<&str>)
    -> Result<Option<T>, E> {
    match text {
        None => { Ok(None) }
        Some(text) => { Ok(Some(text.parse::<T>()?)) }
    }
}