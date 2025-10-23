use std::io;

fn main() {
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    println!("Enter your choice (1 or 2):");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read choice");
    let choice = choice.trim();

    println!("Enter temperature value:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let temp: f64 = input.trim().parse().expect("enterr valid number");

    if choice == "1" {
        let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
        println!("{temp}°C = {fahrenheit}°F");
    } else if choice == "2" {
        let celsius = (temp - 32.0) * 5.0 / 9.0;
        println!("{temp}°F = {celsius}°C");
    } else {
        println!("Please enter 1 or 2.");
    }
}
