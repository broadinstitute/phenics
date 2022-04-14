use clap::{command, Arg};
use crate::error::Error;
use crate::error;

pub(crate) struct Config {
    pub(crate) inputs: Vec<String>,
}

pub(crate) fn get_config() -> Result<Config, Error> {
    const INPUT: &str = "input";
    let app = command!()
        .arg_required_else_help(true)
        .arg(Arg::new(INPUT)
            .short('i')
            .long(INPUT)
            .takes_value(true)
            .value_name("FILE")
            .multiple_values(true)
            .help("Input files; use STDIN if none provided")
        );
    let matches = app.try_get_matches()?;
    let input =
        error::none_to_error(matches.values_of(INPUT),
                             "Need to specify input files")?
            .map(String::from).collect();
    Ok(Config { inputs: input })
}