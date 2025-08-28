use std::{env, process};
use minigrep::{run, Config};

fn main() {  
  let config = Config::build(env::args()).unwrap_or_else(|err| {
    eprintln!("Oops! Grepped into an error: {}", err);
    process::exit(1); // Exitting with non-zero code shows potential error in exec
  });

  println!("Query is: {}", config.query);
  println!("File path is: {}", config.file_path);
  println!();

  if let Err(e) = run(config) {
    eprintln!("Application error: {}!\n Exiting...", e);
    process::exit(1);
  }
}
