use std::io;

fn last_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate().rev() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }

    &s[..]
}

fn main() {
    let mut input = String::new();

    println!("Enter a sentence:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let trimmed = input.trim();

    let word = last_word(trimmed);
    println!("The last word is: {}", word);
}
