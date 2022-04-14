use crate::config::get_config;
use crate::error::Error;

mod config;
mod error;
mod stats;

pub fn run() -> Result<(), Error> {
    let config = get_config()?;
    if config.inputs.is_empty() {
        return Err(Error::from("Need to specify at least one input file."));
    }
    for input in config.inputs {
        println!("{}", input)
    }
    Ok(())
}