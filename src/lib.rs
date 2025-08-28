use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;

  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &contents)
  } else {      
    search(&config.query, &contents)
  };

  for line in results {
    println!("{line}");
  }

  Ok(())
}

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

/// Config instance
impl Config {
  /// Builds a new `Config` object from args
  /// and throws an `Err` in case `args.len() < 3`
  pub fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
    args.next(); // Skipping binary target parameter

    let query = match args.next() {
        Some(args) => args,
        None => return Err("Couldn't get your query!")
    };
    let file_path = match args.next() {
        Some(args) => args,
        None => return Err("Couldn't get your file!")
    };

    let ignore_case = env::var("IGNORE_CASE").is_ok();

    Ok(Config { query, file_path, ignore_case })
  }
}

/// Tests
#[cfg(test)]
mod test {
  use super::*;

  #[test]
  /// Case sensitive search test
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape."; // We do not expect this line as it has a 'D' while duct has a 'd'

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }
  #[test]
  /// Case insensitive search test
  fn case_insensitive() {
    let query = "ruSt";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
  }
}

/// Search functionality (case sensitive)
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|l| l.contains(query))
    .collect()
}

/// Search functionality (case insensitive)
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  
  contents
    .lines()
    .filter(|l| l.to_lowercase().contains(&query))
    .collect()
}