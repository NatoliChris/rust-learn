use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8
}


impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        //Take note with the padding: (:02X) means pad to 2, in hex
        write!(f, "0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}


fn main() {
    //Iterate through a list with .iter()
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 0, blue: 0 },
        Color { red: 255, green: 255, blue: 255 },
    ].iter() {
        println!("{}", *color);
    }
}
