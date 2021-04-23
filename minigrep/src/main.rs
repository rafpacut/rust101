use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_args(&args);
    println!("searching for {} in {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong with reading the file");

    println!("With text:\n {}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_args(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config{query, filename}
}
