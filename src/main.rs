use phenics::run;

fn main() {
  match run() {
    Ok(_) => { /* No-op */ }
    Err(error) => {
      eprintln!("{}", error)
    }
  }
}
