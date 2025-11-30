use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() {
    let output = Command::new("echo")
        .arg("Rust Process Management")
        .output()
        .expect("Failed to run echo");
    let text = String::from_utf8_lossy(&output.stdout);

    let mut file = File::create("output.txt")
        .expect("Failed to create file");

    file.write_all(text.as_bytes())
        .expect("Failed to write to file");

    println!("Wrote to output.txt");
}
