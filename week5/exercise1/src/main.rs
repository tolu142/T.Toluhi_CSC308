use std::io;

fn main() {
    println!("Insert number to factorial:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let r: u32 = input.trim().parse().expect("");

    let factorial = |n: u32| {
        let mut result = 1;
        for i in 1..=n {
            result *= i;
        }
        result
    };

    println!("Factorial of {r} is {}", factorial(r));
}
