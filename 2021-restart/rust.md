# Rust Book Notes

## Terms

*  **Packages**: A Cargo feature that lets you build, test, and share crates
  - Package is one or more crates that provide a set of functionality.
  - Can contain **AT MOST ONE** library crate, any number of binary crates but must have **at least one**.
*  **Crates**: A tree of modules that produces a library or executable
  - Group related functionality together in a scope.
  - `src/main.rs` and `src/lib.rs` are roots.
*  **Modules and use**: Let you control the organization, scope, and privacy of paths
  - Organise code within a crate for readability and reuse.
  - Control privacy of items (public, private, ...)
*  **Paths**: A way of naming an item, such as a struct, function, or module
  - Two forms:
    * *absolute* = starts from crate root using the name or `crate`.
    * *relative* = starts from module and uses `self`, `super`, etc.
  - `use` keyword brings a path into scope.
    - `crate::somename` (crate refers to root, somename is the path)

## Notes

* Program structure:
  - `src/lib.rs` should hold a lot of the logic
  - `src/main.rs` is just the main binary to run calling library logic, without keeping too many variables.
  - Library crates can have tests in a `tests` directory.
    - Unit tests are a separate module (`mod tests` with `#[cfg(test)]`)

## Formatting output

* Implement a display trait for easy formatting for output

```rust
impl fmt::Display for <struct name> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    ...
    write!(f, "{}"...)
  }
}
```

* Debug: `{:?}` or `{:#?}`
* Debug: ``dbg!(......);``

## Using `?`

```rust
write!(f, "{}", ...)?
```

The `?` returns the error if it errors, else it will continue.

## Returns

* Explicit `return` can be used.
* Most common is leaving the last value without a semicolon.

```rust
fn add_one(x: i32) -> i32 {
  x + 1 // No semicolon means to return it.
}
```

### Unit type ()

The unit type `()` is similar to an empty tuple?

## Control flows

### If (standard)

* Standard `if` / `else if` / `else`.
* One liners are allowed: ``let num: u32 = if <condition> { 5 } else { 6 };``

#### IF LET

Combines `if` and `let` in a single statement for concise code.

```rust
if let Some(x) = value {
  // Only works if Some is not None
}
```


### Loop

* No exit condition, needs explicit `break` checks.

```rust
loop {
  do stuff
}
```

### While

```rust
while number != 0 {
  number -=1;
}
```

### For

* For each element in ..
* Note, iterators are _probably_ better for this.

```rust
for el in arr {
  println!("{}", el);
}

// OR

for number in (1..4) {

}
```

### Match

A switch for pattern matching.

```rust
match something {
  Enum::Thing1 => {
    do_things;
    println!("Hello there");
    return_something
  }
  Enum::Thing2 => return_something_else,
}
```

Very useful with options

```rust
match x {
  // NOTE on an Option<T> both *Some* and *None* are required in the match!!
  // Alternatively we can use a catch all (like default)
  None => None,
  Some(i) => Some(i + 1),
}
```

Example of catch alls:

```rust
let dice_roll = 10;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}
```

OR

```rust
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}
```

## Ownership!

* Shallow copy in rust is a "move".
* Ownership passing a value to a function is similar to assigning a value to a variable.
* Returning value transfers ownership.
  * However, that's too tedious so get the reference.
* A reference (`&`) refers to the value but does **not** own it.

Things such as:

```rust
let s1 = String::from("hello");
let s2 = s1;
// ^ s1 is now invalidated as a variable, because ownership has been moved.

// This line will error, because s2 owns s1.
println!("{}, world!", s1);
```

BUT!
This WILL work because integer size is known at compile time and all stored on the stack.
It has the `copy` trait.

```rust
let x = 5;
let y = x;
println!("x = {}, y = {}", x, y);
```
* Deep copy done via ``.clone()``.


## Structs

* Struct update syntax allows setting values from other structs.
  * NOTE: this _moves_ the data, so `struct1` can no longer be used!
* Structs do not need named fields -- they can just be: ``struct Point(i32, i32, i32)``


```rust
// Example of taking values from another struct.
struct2 = StructureThing {
  name: differentName,
  ..struct1
}
```

### Impls

* Implementation of methods and functions for structs go in `impl` blocks.
  * Note; can have multiple `impl` blocks.

```rust
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  // Accessed with rectangle_instance.area()
  fn area(&self) -> u32 {
    self.width * self.height
  }

  // "Associated function"
  // Accessed: Rectangle::square(5)
  fn square(s: u32) -> Rectangle {
    Rectangle {
      width: s,
      height: s,
    }
  }
}
```

## Enums

* Can have different types:
```
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Change(i32, i32, i32),
  Write(String),
}
```

## Modules and privacy

* Need `pub mod` to expose a public mod --> need `pub fn` to access the functions in public.
* `pub` also used with structs, enums, and struct properties, etc.
* Modules and separate files:
  - Can be brought into scope using `use`. (also `pub use`)
  - using a block tells Rust to load the contents of the module from another file with the same name as the module (e.g. ``mod some_file_name``)


## Error Handling

* Result -> OK, Err
  - Result can be used when an error is recoverable (e.g. file doesn't exist -> create one)
  - Use a `match` on `Ok(...), Err(error) ...`.
  - Matching can be done on specific error types (e.g. ``ErrorKind::NotFound``)
* Unwrap can throw errors, so use `expect` -- ``File::open('...').unwrap().expect(....)``
* `panic!` will abort and then unwind the stack.
  - Use `panic = 'abort'` in the cargo config to skip the stack unwinding and leave gc to os.
* Propagating errors
  - Return explicitly: ``return Err(..)``
  - Use the `?` (after a `Result`) operator to propagate the error up.
    * ``File::open("..")?;``
* When to panic?
  - Irrecoverable errors:
    * Bad state is not something that happens often.
    * Code after this point must rely on a _good_ state.
    * No good way to encode the information in the types you use.
  - When failure is expected -> better to use `Result` and handle gracefully.

## Traits
* Behaviour for a type.
* Allow default implementation

Example, a summary trait:

```rust
pub trait Summary {
  fn summarize(&self) -> String {
    String::from("Read More..")
  }
}


impl Summary for MyStruct {
  fn summarize(&self) -> String {
    format!("{}, Read More", self.title)
  }
}
```

## Lifetime annotation syntax

* Uses the `'` (e.g.: `'a` to denote a lifetime.
* Placed after the `&`.

## Testing

* Unit tests -> convention is to have a `mod tests` for that specific package.
* Integration tests are placed in a `tests` directory alongside `src`.
  - This only works for library (`src/lib`) crates.

## Closures

* params in vertical pipes (`|`), can be typed or infer type at compile time.

```rust
let some_closure = |x| {
  do_some_work()
  println!("do something");
  x
}

// Simple, single expression closures aren't don't require brackets.
let simple_closure = |x| x+1;
```

* Example of a struct that caches the value of a closure (so it doesn't need to be run multiple times)
  - Note; this example assumes that it will always get the same value!
```rust
struct CacheMyClosure<T>
where
  T: Fn(u32) -> u32,    // Defines the type bound on the generic T.
{
  calculation: T,       // This will be the closure function to calculate
  value: Option<u32>,
}

// Implementation
impl <T> CacheMyClosure<T>
where
  T: Fn(u32) -> u32,
{
  fn new(calculation: T) -> CacheMyClosure<T> {
      CacheMyClosure {
        calculation,
        value: None,
      }
  }

  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        // Run the closure by calling it: ()()
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
    }
  }
}
```

* Improving with a map?

	```rust
	use std::collections::HashMap;

	struct CacheWithMap<T>
	where
			T: Fn(u32) -> u32,
	{
			calculation: T,
			values: HashMap<u32, u32>,
	}

	impl <T> CacheWithMap<T>
	where
			T: Fn(u32) -> u32,
	{
			fn new(calculation: T) -> CacheWithMap<T> {
					CacheWithMap {
							calculation,
							values: HashMap::new(),
					}
			}

			fn value(&mut self, arg: u32) -> u32 {
					// Get the value from the map
					match self.values.get(&arg)  {
							Some(v) => *v,
							None => {
									// Run the calculation
									let v = (self.calculation)(arg);
									self.values.insert(arg, v);
									v
							}
					}
			}
	}
	```

* Closures are able to access the environment around them as part of scope.

	```rust
	let x = 4;
	let equal_x = |z| z == x;
	```

* Function traits:
	- **FnOnce** - consumes the variable it captures from enclosing scope.
		* The `Once` part of the name suggests it can't take ownership more than once. (only callable once).
	- **FnMut** - Mutable; is able to change the environment.
	- **Fn** - Borrows from the environment immutably.
* Note also, the `move` keyword in closures may still let them implement `Fn` or `FnMut`, because types are determined by what the closure **does** rather than how it captures variables.


## Iterators!

* `.iter()` doesn't do anything until it is called; It provides an iterator over *immutable* references.
	- Iterators are lazy and will do nothing until consumed.
	- `into_iter` will consume and turn it into mutable references.
* Implementing an iterator requires the `type Item` defined to be used with `next()`.
* When making a `.iter()` it needs to be `mut` because `.next()` consumes the item from the iterator.
* Methods can consume the iterator
	- `.next()` - A 'consuming adaptor' that uses up the iterator and returns the next.
	- `.sum()`, ...,
	- `.collect()` (will consume the iterator and return a vector)
	- etc..

### Creating own iterator through Iterator trait

Example: a finite counter that counts up to 10.

```rust
struct MyCounter {
	count: u32,
}

impl MyCounter {
	fn new() -> MyCounter {
		MyCounter { count: 0 }
	}
}

// Implement the iterator trait
impl Iterator for Counter {
	type Item = u32; 						// As in docs, requires an item type to be defined!

	fn next(&mut self) -> Option<Self::Item> {
		if self.count < 10 {
			self.count += 1;
			Some(self.count)
		} else {
			None
		}
	}
}
```
