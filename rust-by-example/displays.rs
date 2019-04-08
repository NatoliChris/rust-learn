/*
 * Implementing our own Display
 *  > Equivalent to toString
 */

use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct MinMax(i32, i32);

//Now let's implement the display:
impl fmt::Display for Structure {
    // Make a `trait`, it MUST be like this for Display:
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write is very similar to println
        write!(f, "{}", self.0)
    }
}

/* Implementing my own structure */
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}




fn main() {
    println!("Debug: {:?}", Structure(2));
    println!("Display: {}", Structure(2));

    println!("Debug: {:?}", MinMax(1, 2));
    println!("Display: {}", MinMax(1, 2));
}
