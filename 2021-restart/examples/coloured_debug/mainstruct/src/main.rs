use std::fmt;
extern crate colored;

use colored::*;
use coloured_debug_derive::ColouredDebug;

#[derive(ColouredDebug)]
pub struct HelloWorld {
    id: u32,
    name: String,
}

fn main() {
    let a = HelloWorld{ id: 1, name: String::from("hello")};
    println!("{:?}", a);
}
