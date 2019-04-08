use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

static DERP: &'static str = "DERP herp herpa derpa derp herpaderp\n aslkdjalkdsj askjdasd \n";

fn main() {
    //spawn wc
    let process = match Command::new("wc")
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn() {
                            Err(why) => panic!("Couldn't spawn: {}", why.description()),
                            Ok(process) => process,
                        };
    
    match process.stdin.unwrap().write_all(DERP.as_bytes()) {
        Err(why) => panic!("Can't write: {}", why.description()),
        Ok(_) => println!("Sent!"),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read output: {}", why.description()),
        Ok(_) => println!("Wc: {}", s),
    }

}
