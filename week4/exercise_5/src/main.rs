use std::io;

struct Student {
    name: String,
    score: f32,
}

impl Student {
    fn new(name: String, score: f32) -> Self {
        Self { name, score }
    }

    fn evaluate(&self) {
        if self.score >= 40.0 {
            println!("{} passed the course with a score of {:.2}.", self.name, self.score);
        } else {
            println!("{} failed the course with a score of {:.2}.", self.name, self.score);
        }
    }
}

fn main() {
    let mut name_input = String::new();
    let mut score_input = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name_input).expect("Failed to read name");

    println!("Enter student score:");
    io::stdin().read_line(&mut score_input).expect("Failed to read score");

    let name = name_input.trim().to_string();
    let score: f32 = score_input.trim().parse().expect("Please enter a valid number");

    let student = Student::new(name, score);
    student.evaluate();
}
