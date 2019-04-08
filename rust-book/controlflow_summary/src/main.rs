use std::io;

fn convert_to_celcius() {
    const FAHRENHEIT_BASE: u8 = 32;
    const FAHRENHEIT_MUL: f32 = 1.8;

    println!("Please enter a number in Fahrenheit");

    let mut f_temp = String::new();

    io::stdin().read_line(&mut f_temp)
        .expect("Failed to read line");

    let f_temp: f32 = f_temp.trim().parse()
        .expect("Please type a number");

    let c_temp: f32 = (f_temp * FAHRENHEIT_MUL) + f32::from(FAHRENHEIT_BASE);

    println!("{} F = {} C", f_temp, c_temp);
}

fn fibonacci(x: u8) -> u8 {
    if x == 0 {
        0
    } else if x == 1 {
        1
    } else {
        fibonacci(x-1) + fibonacci(x-2)
    }
}

fn fibonacci_array(x: u8) -> u8 {
    let mut fibonacci_numbers = vec![0; (x + 1) as usize];

    fibonacci_numbers[0] = 0;
    fibonacci_numbers[1] = 1;

    for i in 2..=x {
        fibonacci_numbers[i as usize] = fibonacci_numbers[(i - 1) as usize] + fibonacci_numbers[(i - 2) as usize];
    }

    fibonacci_numbers[x as usize]
}

fn main() {
    println!("Running conversion");
    convert_to_celcius();

    println!("Please enter N");
    let mut n = String::new();
    io::stdin().read_line(&mut n)
        .expect("Failed to read line");
    let n: u8 = n.trim().parse().expect("Please type a number");

    println!("{}th fibonacci (recursive) = {}", n, fibonacci(n));
    println!("{}th fibonacci (array) = {}", n, fibonacci_array(n));
}

