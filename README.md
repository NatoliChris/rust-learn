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





