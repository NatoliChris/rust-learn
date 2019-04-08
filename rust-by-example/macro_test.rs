macro_rules! test_mac {
    () => (
        println!("Hello!");
    )
}

macro_rules! test_arg {
    ($hello:expr) => (
        println!("{}", $hello);
    )
}

fn main() {
    test_mac!();
    test_arg!("Hai");
}
