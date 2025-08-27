use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    // let config = parse_config(&args);
    let config = Config::new(&args);

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
    /// Generates a new `Config` object from args
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments!. Exiting...");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        return Config { query, file_path };
    }
}
