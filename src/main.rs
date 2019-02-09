use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // with eprintln! we print to stderr instead of
        // stdout, as it should be
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        // Same here
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

