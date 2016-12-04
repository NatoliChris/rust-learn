# Learning Rust

## Notes

### Hello, World! 

Macros delimited with ``!``

```
println!("Hello, World");

let x = 5 * 5
println!("Testing {}", x)\
```

Also have formatted printing

```
println!("Printing can be numbered {0} {1} {0}, but also can be {:b}", 1, 2, 4)

println!("Padding: {number:>width$}", number=1, width=6);

//Dealing with precision of numbers:
println!("{number:.precision$}", precision=3, number=3.141592);
```

Good documentation: [https://doc.rust-lang.org/std/fmt/](https://doc.rust-lang.org/std/fmt/)

#### Printing to Debug:

Debugging is imporant!

```
println!("Debug uses {:?}", ":?");

```

### Library

To import a library ``use __libraryname__`` e.g. ``use std::fmt;``

### Making a struct display-able

```
// Derive debug allows it to print in debug
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Complex {
	real: f64,
	imag: f64
}

impl fmt::Display for Structure {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
		// Note : no semicolon = returns
		write!(f, "{}", self.0)
	}
}


impl fmt::Display for Complex {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} + {}i", self.real, self.imag)
	}
}

fn main() {
	let x = Complex { real: 3.3, imag: 2.2 };

	println!("{}", x);
}

```


