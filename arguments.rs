use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!(" Give me some arguments m8 ");
        },

        _ => {
            println!(" Args {:?}", args);
        },
    }
}
