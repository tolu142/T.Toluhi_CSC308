use std::process::Command;

fn main() {
    let run_child = || {
        Command::new("echo")
            .arg("Hello from child process!")
            .spawn()
            .expect("Failed to spawn child process")
            .wait()
            .expect("Child process failed to run");
    };

    
    run_child();
}
