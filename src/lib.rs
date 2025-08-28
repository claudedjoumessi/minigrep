use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
      println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

/// Config instance
impl Config {
    /// Builds a new `Config` object from args
    /// and throws an `Err` in case `args.len() < 3`
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!. Exiting...");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
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
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

/// Search functionality (case insensitive)
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  let query = query.to_lowercase(); // lowercasing here...

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) { // to borrow here
      results.push(line);
    }
  }

  results
}