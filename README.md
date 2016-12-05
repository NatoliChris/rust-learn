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

### Iterating through a list with for

List items use ``.iter()`` to iterate through.

Can also get the count with ``.iter().enumerate()``

e.g.

```

for (index, v) in vec.iter().enumerate() {
	if index != 0 {
		try!(write!(f, ", "));
	}
	try!(write!(f, "{}: {}", index, v));
}

```

### Tuples

Collection, just like other languages.

Can be used as function arguments and return values.

```

fn reverse(pair: (i32, bool)) -> (bool, i32) {
	let (integer, boolean) = pair;
	(boolean, integer)
}

let (a,b,c,d) = tuple_variable;

```

### Arrays

Need to be initialized, and the size is set:

```
let xs: [type; len] = [/* types here */];

//can also be set in bulk

let xy: [i32; 500] = [10; 500]; /* Sets every value to 10 */

```

---

### Custom Types

- Can ``enum`` for enumerations
- Can ``struct`` for structs


#### Enums:

```
enum NameHere {
	items,
	in_the,
	enum,
}

```

Enums can make good use of ``use``. E.g.:

```
enum Person {
	A, 
	B, 
	C,
}

use Person::{A, B, C};

//Equivalent to `Person::A`
let me = A;

match me {
	Person::A => println!("Hello me!"),
	B => println!("I'm a B? bzzz"),
}
```

#### Constants

- ``const``: unchangeable
- ``static``: mutable ``mut`` with static lifetime

```
// 'static means it is mutable
static LANGUAGE: &'static str = "Rust";
//can't change a constant!
const THRESHOLD: i32 = 10;
```

### Variable Binding

Values can be bound to variables:

```
let _unused_variable = 3u32;
let used_variable = 1u32;
let copied_int = used_variable;
```

#### Mutability:

Variable bindings immutable by default unles ``mut`` is used.

```
let _immutable_binding = 1;
let mut mutable_binding = 1;

mutable_binding += 1;		// All good
_immutable_binding += 1;	// Error!
```

#### Scope / Shadow

Scope can be in blocks

```
{
	let temp_var = 2;
	
}
```

Shadowing is redeclaring a variable

```
let long_var = 1;
let long_var = 'a';
```

### Casting

Use the ``as`` keyword. 

```
println!("1000 as u16 is: {}", 1000 as u16);
```

#### Alias types

Can be used to give names to existing types. 

```
type NanoSecond = u64

let nanoseconds: NanoSecond = 5 as u64
```

### Expressions 

You can use expressions in variables

```
let x = 5;

let y = {
	let x_squared = x * x;
	let x_cube = x_squared * x;

	//Retun the following, will be assigned as `y`
	x_cube + x_squared + x
};
```

Some are special, suppresses expression.

```
// Will be assigned as () not any number.
let z = {
	2 * x;
};
```

---

### Control Flow

Similar to other languages

```
if condition {

} else if second_condition {

}

```

Loops are the same, but can have infinite loops

```
//infinite loop
loop {
	
	if condition {
		//break out of the loop
		break;
	}
}
```

Can also label loops
```
'outer: loop {
	'inner loop {
		//Break out of the outer loop.
		break 'outer;
	}
}
```
