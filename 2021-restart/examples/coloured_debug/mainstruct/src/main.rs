use std::fmt;
extern crate colored;

use colored::*;
use coloured_debug_derive::ColouredDebug;

#[derive(ColouredDebug)]
pub struct HelloWorld;

fn main() {
    println!("{:?}", HelloWorld{});
}
