use clap::{command, Arg};
use crate::error::Error;

pub(crate) struct Config {
    pub(crate) input: Option<Vec<String>>,
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
        matches.values_of(INPUT).map(|values| { values.map(String::from).collect() });
    Ok(Config { input })
}