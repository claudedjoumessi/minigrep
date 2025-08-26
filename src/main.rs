use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Query is: {query}");
    println!("File path is: {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");

    println!("With contents\n: {contents}")
}
