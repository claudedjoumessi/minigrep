use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Query is: {}", config.query);
    println!("File path is: {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read file");

    println!("With contents\n: {contents}")
}

struct Config {
    query: String,
    file_path: String,
}

/// Config parser
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    return Config { query, file_path };
}
