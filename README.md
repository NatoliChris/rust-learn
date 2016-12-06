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

For loops:

```
//Inclusive .. exclusive
for n in 1..101 {
	println!("{}", n);
}
```

Matching with Bindings:

```
match age {
	0 => println!("I don't exist!");
	n @ 1 ... 12 => println!("I'm a real boy!");
	n @ 13 ... 19 => println!("I'm a-live....");
	_ => println!("I'm a default");
}
```


``if let`` can be used to assign and check:

```
let number = Some(7);

if let Some(i) = number {
	println!("Matched {:?}" , i);
}
```

---

### Functions

Allow closures

```
// Increments through function and closure
fn increment (i:32) -> i32 {i + 1};

// Increment just as closure
let closure_inferred = |i| i + 1;
```

---

### Modules

Modules have public and private (default).

```

mod my_mod {
	fn private_function() {
		println!("Shh I'm private, only called with `my::private_function()`);
	}

	pub fn pub_function() {
		println!("Hi I'm a public function `my::function()`);
	}

	pub mod nested {
		pub fn function() {
			println!("I'm nested");
		}
	}
}
```

#### Super and Self.

Super is used to call parent scope outside, 
Self is used to refer to the current module

Refer to [supes.rs](supes.rs) to find an example

### Libraries

Easily create library files:

```
$ rustc --create-type=lib test.rs
libtest.rlib
```

Then to use them in a rust file:

```
extern crate test;
```

### Configs

Can help to specify that certain conditions are met.

Example, only compile on linux:

```
#[cfg(target_os = "linux")]
fn some_function_only_linux() {
}
```

--

Or certain condition in flags:
```
#[cfg(some_cond)]
fn condition_function() {
	println!("Condition");
}
```

Then to compile:

```
$ rustc --cfg some_cond named.rs
$ ./named
Condition
```

---

### Generics

Simple like other languages.

Structs

```
struct Random(A);
```

Functions

```
fn foo<T>(T) {

}
```

Allows explicit type to generic.

```
struct SGen<T>(T);

generic::<char>(SGen('a'));
```

---

### Traits

Collection of methods for unknown type that can be implemented for any data type.

```

trait Foo {
	// functions here
}

struct Bar { ... }

impl Foo for Bar {
	// override the functions
}

```

Basic implementations of traits with ``#[derive]``.
Allow for a list of derivable traits:

- Comparions 
- Clones
- Copy
- Hash
- Default
- Zero
- Debug

e.g.

```
#[derive(Debug, Hash)]
```

---

### Macro rules

Macros end with a ``!``, expanded into source code compiled. 
Just like C's ``#DEFINE``

Example: A macro that prints hello

```

macro_rules! say_hello {
	() => ( 
		//takes no arguments and prints hello
		println!("Hello!");
	)
}

```

Please see [macro_test.rs](macro_test.rs) for my testing of macros.


---

### Error Handling

#### Panic

Similar to golang, panic prints error:

```
panic!("error messageee!");
```

#### Option / Unwrap

``Option`` is an enum, either ``Some(T)`` or ``None``. 

Example:

```
fn test_me(arg: Option<&str>) {
	let a = arg.unwrap();
	//Unwrap will panic if it receives a `None`
}

```

#### try!

Try is like unwrap but will return an ``Err``.

```
let elem = try!(array[0]);
```


#### Definining custom error type

Present a nice error message to the user, holds critical information.

```
Err(EmptyVec);							// Good
Err("At least one element".to_owned()); //bad
```

---

### Threads
Has a ``spawn`` function:

```
thread::spawn(move || {
	println!("Thread!");
})

```

See [threads_test.rs](threads_test.rs)

### Channels - just like Go

Two endpoints, ``Sender`` and ``Receiver``.

Need to define what information is being sent.


The ``send(x)`` method on the sender is *non blocking*
The ``recv()`` method on the receiver **IS** blocking.

```
let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

// Loop
	let thread_tx = tx.clone(); // clone the sender
	thread_tx.send(i).unwrap();
//End loop
```

See [channel_test.rs](channel_test.rs)


---

### File IO

#### Open

Open used for read only. File owns resource, takes care of closing file when dropped.

```
let path = Path::new("/path/to/file");
let mut file = match File::open(&path) {
	// description of io::Error is a string
	Err(why) => panic("{}: {}", display, why.description()),
	Ok(file) => file,
};
```

When the file goes out of scope, it is closed.

#### Create

Write only mode, if the file exists it is **Destroyed!**

```
let path = Path::new("/path/to/outfile.txt");

let mut file = match File::create(&path) {
	Err(why) => panic(....),
	Ok(file) => file,
};

```

See [fileio.rs](fileio.rs)


---

### Working with children - Child Processes

Works with child processes, ``process::Output`` is the output of finished.
``process::Command`` is a process builder. 

```
use std::process::Command;

let output = Command::new("rustc")
	.arg("--version")
	.output().unwrap_or_else(|e| {
		panic!("Failed to execute: {}", e)
	});

```

See [simplechild.rs](simplechild.rs)

#### Processes also need pipes

Similar to other languages, especially C.

```
use std::process::{Command, Stdio};

Command::new("wc")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
```

See [pipedchild.rs](pipedchild.rs)

#### Waiting ....

Need to wait for a process to finish, call ``Child::wait``.

```
use std::process::Command;

let mut child = Command::new("sleep").arg("5").spawn().unwrap();
let _result = child.wait().unwrap();
```


See [waitingchild.rs](waitingchild.rs)

---

### Program Args 

Command line arguments through iterator

```
use std::env;

let args: Vec<String> = env::args().collect();
```

#### Parsing Arguments

Suggested to use matching because it is simple

See [arguments.rs](arguments.rs)


---

## Miscellaneous 

### Documentation

Rustdoc similar to Javadoc

``///`` to start the rustdoc and it supports markdown!

### Testing

``#[test]`` signifies the start of a unit test, must be carefully configured:

```
#[cfg(not(test))]
fn main() {
	println!("You must run this as a test file!");
}

#[cfg(test)]

mod test {
	#[test]
	fn .... {
		...
	}
}

```

Need to compile with testing flag:

```
rustc --test testingfile.rs
```

See [testing.rs](testing.rs)
