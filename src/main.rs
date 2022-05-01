use phenics::run;

fn main() {
  match run() {
    Ok(_) => { eprintln!("Done!") }
    Err(error) => { eprintln!("{}", error) }
  }
}
