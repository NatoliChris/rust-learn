use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Dereference self, create a reference to vec
        let List(ref vec) = *self;
        try!(write!(f, "["));

        //iterate through vec
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                try!(write!(f, ", "));
            }
            try!(write!(f, "{}: {}", count, v));
        }

        write!(f, "]")
    }
}


fn main() {
    let v = List(vec![1,2,3]);
    println!("{}", v)
}
