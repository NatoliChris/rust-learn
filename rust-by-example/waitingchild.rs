use std::process::Command;

fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _res = child.wait().unwrap();

    println!("Fin.");
}
