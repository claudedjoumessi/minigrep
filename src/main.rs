use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Oops! Grepped into an error: {}", err);
        process::exit(1); // Exitting with non-zero code shows potential error in exec
    });

    println!("Query is: {}", config.query);
    println!("File path is: {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read file");

    println!("With contents\n: {contents}")
}

struct Config {
    query: String,
    file_path: String,
}

/// Config instance
impl Config {
    /// Builds a new `Config` object from args
    /// and throws an `Err` in case `args.len() < 3`
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!. Exiting...");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
