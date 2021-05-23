use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    println!("searching for {} in {}", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Error encountered: {}",e);

        process::exit(1);
    };
}
