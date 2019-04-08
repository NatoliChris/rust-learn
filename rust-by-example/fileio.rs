use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {

    //Write to the file
    {
        let path = Path::new("testout.out");
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't Create {}: {}",
                               display,
                               why.description()),
            Ok(file) => file,
        };

        let asdf = "QWERTYUIOP lorem ipsum derp derp derp herp";

        match file.write_all(asdf.as_bytes()) {
            Err(why) => panic!("Couldn't write {}: {}",
                              display,
                              why.description()),
            Ok(_) => println!("Success! {}", display),
        }
    }

    //Read the file
    {
        let path = Path::new("testout.out");
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("Couldn't open{}: {}",
                               display,
                               why.description()),
            Ok(file) => file,
        };

        let mut s = String::new();

        match file.read_to_string(&mut s) {
            Err(why) => panic!("Couldn't read! {}: {}",
                               display,
                               why.description()),
            Ok(_) => println!("{} contains:\n {}", display, s),
        }

    }

}
