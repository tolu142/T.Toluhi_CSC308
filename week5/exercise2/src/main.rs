fn main() {

    let numbers: Vec<i32> = (1..=20).collect();

    let is_even = |x: &i32| x % 2 == 0;

    let evens: Vec<i32> = numbers.iter().cloned().filter(is_even).collect();

    println!("Even numbers from 1 to 20: {:?}", evens);
}
