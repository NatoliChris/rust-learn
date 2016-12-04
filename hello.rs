fn main() {
    println!("Hello, World!");

    /* Formatted printing: */
    println!("The {adjective} {adjectivetwo} {noun} runs over the lazy {second}",
             adjective="quick",
             adjectivetwo="brown",
             noun="fox",
             second="dog"
             );

    // Padding:
    println!("{number:>width$}", number=1, width=6);

    //Decimal Places:
    let Pi = 3.141592353589793;
    println!("Hello {:.*}", 3, Pi);

    //Or more easily understood:
    println!("{number:.precision$}", precision=3, number=Pi);
}
