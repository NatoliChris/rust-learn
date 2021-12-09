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

* Assignment is a *move*, where the compiler tracks ownership of the data.
  - The `Copy` trait allows copying on assignment.
  - `.clone()` performs a deep copy (performs memcpy)
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

* Iterators are quick!
  - Rust uses the *zero-cost abstraction* : no additional runtime overhead.
  - C++ implementations obey the zero-overhead principle.
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
  type Item = u32;            // As in docs, requires an item type to be defined!

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

## Cargo and Crates

* Specify different release and optimisations in the `Cargo.toml`
  - By default, opt level is 0; sets the optimisation level.
  ```rust
  [profile.dev]
  opt-level = 0

  [profile.release]
  opt-level = 3
  ```
* Publish the crates on cargo.io for others to use.
    - ``cargo publish`` will publish the crate.
    - ``cargo yank --vers 1.0.1`` removes the version specified from crates.io

### Workspaces

* Set of packages that share some `Cargo.lock` and output directory.

```bash
cargo new add
```

In `Cargo.toml`:

```
[workspace]

members = [
    "add",
]
```

Will result in;

```
├── Cargo.lock
├── Cargo.toml
├── add
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

* Also works with multiple workspaces.
* Cargo.toml points by using a relative path inside the second workspace
  - E.g.: ``add = { path = "../add" }``

## Documentation

* Rust doc `///` supports markdown formatting.
    - `//!` adds documentation to the item that contain sthe comment, rather than documentation to items following the comments.w
        * Used for crate as a whole (for example).
* Compile with ``cargo doc`` (e.g. ``cargo doc --open`` will build HTML for current project.)
* Specific headings to be used for documentation:
  - `# Examples` - sections that will be titled examples.
  - `# Panics` - Section about scenarios that could cause a panic.
  - `# Errors` - Section that describes errors that might return with a `Result` type.
  - `# Safety` - If function is `unsafe` --> why?
* Example code is also tested with `cargo test` to see that it builds!
* Modules
  - ``pub mod ... { pub fn ... }`` will export to the documentation.
  - Can also use ``pub mod ... ; pub use self::mod::function``
    - Pub use gives you flexibility in structure and crate as well as decouples internal structure to what is presented to users.

## Smart Pointers

* Similar concept to C++ smart pointers.
* Overview
  - `Rc<T>` - reference counter -> multiple owners of the same data;
    - `Box` and `RefCell` have single owners.
  - `Box<T>` - immutable / mutable borrows checked at compile time.
    - `Rc` only allows immutable checked at compile time.
    - `RefCell` allows immutable/mutable checked at **runtime**.
  - `RefCell<T>` - mutable borrows checked at runtime that also allows mutation.

### Box<T>

* Allow you to store data on heap, rather than stack.
* Included in prelude, no need to require `use` for anything.
* Like a smart pointer because implements `Deref` and has `Drop` trait.
* Minimal performance overhead.
* Uses:
  - Type with unknown size at compile time but requires context of known size.
  - Large amount of data to transfer ownership but don't want copy.
  - Own a value and only care that type implements a particular trait (rather than is a particular type)
* Useful for *recursive types*.
  - Example: a `cons` list (`cons` is used as a construct function taken from Lisp)
  ```rust
  // Note: the box is useful because otherwise "List" type would have infinite size!
  //        this is because Box is a pointer and has a size, therefore can be easily known.
  enum List {
    Cons(i32, Box<List>),         // Represents the list with Box so recursive will work.
    Nil,
  }

  fn main() {
    let lst = Cons(1, Cons(2, Cons(3, Nil)));
  }
  ```

### Treating smart pointers as regular references with Deref trait.

* `Deref` trait customises the behaviour of `*` dereference. (immutable)
  * ``DerefMut`` trait allows you to override the `*` on mutable references.

Example: A fake box that stores data on the stack as an example.

```rust
use std::ops::Deref;

struct MyFakeBox<T>(T);

impl<T> MyFakeBox<T> {
  fn new(x: T) -> MyFakeBox<T> {
    MyFakeBox(x)
  }
}

impl<T> Deref for MyFakeBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

// Now usage
fn main() {
  let x = 5;
  let normal_box = Box::new(x);

  assert_eq!(5, *normal_box);

  // Now with new box
  let my_box = MyFakeBox::new(x);
  assert_eq(5, *my_box);
}
```

* Deref coercion makes it possible to call a function where the arguments don't quite match but can be converted.
  - E.g. `hello(name: &str)` can take a `String` since `String.deref()` returns an `&str`.
  - Very helpful for being able to write without explicitly specifying references and dereferences.
* Rust does deref coercion when it finds types and trait implementations in three cases:
  - From `&T` to `&U` when `T:Deref<Target=U>` (T derefs in to U)
  - From `&mut T` to `&mut U` when `T:DerefMut<Target=U>` (T's mutable dereference turns into type U).
  - From `&mut T` to `&U` when `T: Deref<Target=U>` (A mutable T dereferences into immutable type U?)
      - > Rust will also coerce a mutable reference to an immutable one. But the reverse is not possible: immutable references will never coerce to mutable references. Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn't compile). Converting one mutable reference to one immutable reference will never break the borrowing rules.

### Cleanups

* Lower level languages need explicit `free`
* `Drop` trait requires the `drop` function to be implemented. (destructor)
  - Facilitates the clean up of the data (drop when instance goes out of scope)
* Early drop can be done with `std::mem::drop` using ``drop(instance_here)``

### Rc<T> - Reference counted smart pointer.

* ``Rc<T>`` is a smart poiinter to know how many references to see whether still in use.
* From standard lib: ``use std::rc::Rc``
* Useful in sharing data.
  - ``Rc::new(some_thing_here)``.
* Cloning `Rc::clone()` increases the reference count.
  - Count can be found with ``Rc::strong_count(your_rc_var_here)``

### RefCell<T> and interior mutability

* Interior mutability is a design pattern in rust -- mutate data even if immutable references to that data exist.
* Mutating this data requires the use of `unsafe`.
* `RefCell<T>` represents ownership of the data.
  - Onwership and borrowing rules are enforced during *runtime*, not compile time like `Box<T>`
  - Why? Some safe-memory operations may be allowed in runtime, but not at compile time.
    * Note this may be because the compiler doesn't understand, so `RefCell` is used.
* Interior mutability (example: mock objects)
  - `borrow` and `borrow_mut`.
  - Panic occurs at runtime.
* `Rc<T>` and `RefCell<T>` can be combined to provide multiple owners of mutable data.
  ```rust
  let value = Rc::new(RefCell::new(5));
  let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
  value.borrow_mut() += 10;
  ```

### Reference cycles can leak memory!

* `Rc<T>` and `RefCell<T>` can lead to some memory leaks, despite the rust memory safety.
* A `Weak<T>` reference might be dropped.
* Calling `upgrade` on a `Weak<T>` will return `Option<Rc<T>>`.
* Count can be seen as `strong_count` and `weak_count`.
* A weak can be made by ``Weak<...>`` and ``Weak::new()``

## Concurrency

* Creating threads
  - ``use std::thread`` then ``thread::spawn(||...)`` (it's a closure)
* Waiting to finish uses a `JoinHandle`
  - ``handle = thread::spawn...`` then ``handle.join().unwrap()``
  - ensures that the thread finishes. (normal thread.join as other langs)
* To move values into the thread, you can use `move` : ``thread::spawn(move || {}....``

### Message passing

* ``std::sync::mpsc`` (multiple producers, single consumer) message passing channel.
* Sending a value into the channel will transfer ownership.
* Clone transmitters for channels to use in multiple threads.

### Mutex

* Rust ``std::sync::Mutex`` has a `Mutex<T>` used like: ``Mutex::new(some_thing)``
  - To use the mutex: ``mutex_instance.lock().unwrap()``
  - Returns a `LockResult`, which is why we need to `unwrap`.
* To share between threads use `Arc`!
  - ``Arc::new(Mutex::new(...))`` then in the thread ``Arc::clone(&mutex_instance)``

```rust
let myCounter: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));

for _ in 0..10 {
  let thread_counter = Arc::clone(&counter);
  let handle = thread::spawn(move || {
    let mut num = thread_counter.lock().unwrap();
    *num += 1;
  });
}

// ...
```

### Send and Sync Traits

* The `Send` trait indicates that ownership can be transferred between threads.
  - *Almost* all types implement this trait, but there are exceptions (i.e. `Rc<T>`)
* The `Sync` trait indicates that it is safe to be **referenced** from multiple threads.
  - Any type `T` is `Sync` *if* `&T` is `Send`.
