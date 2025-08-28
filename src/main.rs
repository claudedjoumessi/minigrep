use std::{env, process};
use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Oops! Grepped into an error: {}", err);
        process::exit(1); // Exitting with non-zero code shows potential error in exec
    });

    println!("Query is: {}", config.query);
    println!("File path is: {}", config.file_path);
    println!();

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}!\n Exiting...", e);
        process::exit(1);
    }
}


