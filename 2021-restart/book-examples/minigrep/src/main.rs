use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // NOTE: leaving old code for comparison in the examples.
    // This version:
    //  - Used `collect` to get a slice of strings to pass to config
    // Get the arguments
    // let args: Vec<String> = env::args().collect();
    // let cfg = Config::parse(&args).unwrap_or_else(|err| {
    //     eprintln!("Faield to parse arguments: {}", err);
    //     process::exit(1);
    // });

    // Get the env args directly to lend to the config.
    let cfg = Config::parse(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to parse args! {}", err);
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
