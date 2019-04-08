fn main() {

    let x = 5;
    let y = {
        let x_squared = x * x;

        x_squared + x
    };

    let z = {
        2 * x;
    };

    println!("y is {}", y);
    println!("z is {:?}", z);
}
