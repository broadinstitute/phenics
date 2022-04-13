use crate::config::get_config;
use crate::error::Error;

mod config;
mod error;

pub fn run() -> Result<(), Error> {
    let config = get_config()?;
    match config.input {
        None => { println!("No input files provided.") }
        Some(inputs) => { println!("{} files provided", inputs.len()) }
    }
    Ok(())
}