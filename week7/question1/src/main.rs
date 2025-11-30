use std::process::{Command};
use std::thread;
use std::time::Duration;

fn main() {
    let sleep_child = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("Failed to spawn sleep process");
    println!("Spawned 'sleep 5' with PID {}", sleep_child.id());

    let ls_child = Command::new("ls")
        .arg("-la")
        .spawn()
        .expect("Failed to spawn ls -la process");
    println!("Spawned 'ls -la' with PID {}", ls_child.id());

    let echo_child = Command::new("echo")
        .arg("Hello from child")
        .spawn()
        .expect("Failed to spawn echo process");
    println!("Spawned 'echo' with PID {}", echo_child.id());

    // Keep program alive so you can check with ps/top/pstree
    println!("Main program sleeping for 5 seconds so child processes remain visible");
    thread::sleep(Duration::from_secs(5));
}

