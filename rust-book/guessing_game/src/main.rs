use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // Generate the secret number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Loop creates infinite loop
    loop {
        println!("Make your guess. Between 1 and 100");

        // Store the guess as a mutable string.
        let mut g = String::new();

        // Good coding standard to start a newline to break long lines.
        io::stdin().read_line(&mut g)
            .expect("Failed to read line");

        let g: u32 = match g.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Match has arms that it compares; Similar to switch statement.
        match g.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!(":tada:");
                break;
            }
        }
    }
}
