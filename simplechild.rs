use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("Failed to exec: {}", e)
        });

    //Error checking
    
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("Rustc output: {}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        println!("Rustc Failed: {}", s);

    }
}
