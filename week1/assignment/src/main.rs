use std::io;

fn main() {
    println!("Smart Energy Company Billing Calc");
    println!("");
    println!("Enter your energy consumption in kWh:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let usage: f64 = input.trim().parse().unwrap_or(0.0);

    let rate = if usage > 200.0 {
        30.0
    } else if usage > 100.0 {
        25.0
    } else {
        20.0
    };

    let total = usage * rate;

    println!("");
    println!("Energy Used: {} kWh", usage);
    println!("Rate Applied: ₦{} per kWh", rate);
    println!("Total Bill: ₦{}", total);
}