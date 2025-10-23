use std::io;

fn main() {
    println!("Enter total bill amount:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    let bill: f64 = input.trim().parse().expect("enter  valid number");

    let discount = if bill > 10_000.0 {
        0.15
    } else if bill > 5_000.0 {
        0.10
    } else {
        0.0
    };

    let discount_amount = bill * discount;
    let final_amount = bill - discount_amount;

    println!("Discount applied: {}%", discount * 100.0);
    println!("Final Bill: ₦{:.2}", final_amount);
}
