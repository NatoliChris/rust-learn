//extern crate time;

use std::time::{SystemTime, Instant};
//use time::{strftime};

fn main() {
    let t = SystemTime::now();
    
    //let a = strftime("%T", t);
    println!("{}", t.as_secs());
}
