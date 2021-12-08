use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Get the arguments
    let args: Vec<String> = env::args().collect();

    let cfg = Config::parse(&args).unwrap_or_else(|err| {
        eprintln!("Faield to parse arguments: {}", err);
        process::exit(1);
    });

    match minigrep::run(cfg) {
        Ok(()) => (),
        Err(x) => {
            eprintln!("Error running main: {}", x);
            process::exit(1);
        }
    }
}
