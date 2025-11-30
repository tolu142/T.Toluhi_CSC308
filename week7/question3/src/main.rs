use std::process::{Command, Stdio};
use std::{thread, time::Duration};

fn main() {

    let mut child = Command::new("ping")
        .arg("google.com")
        .stdout(Stdio::piped())   
        .spawn()
        .expect("Failed to start ping");

    println!("Spawned ping with PID: {}", child.id());
    println!("You can now open another terminal and run: top | grep ping");

    // Wait 5 seconds
    thread::sleep(Duration::from_secs(5));

    println!("Killing ping process...");
    let _ = child.kill();


    let status = child.wait().expect("Failed to wait on child");

    println!("Ping process exited with: {}", status);
}
