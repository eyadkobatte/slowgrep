use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = slowgrep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(error) = slowgrep::run(config) {
        eprintln!("Error running application: {error}");
        process::exit(1)
    };
}
