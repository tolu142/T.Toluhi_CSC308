use std::io;

struct SentenceAnalyzer {
    sentence: String,
}

impl SentenceAnalyzer {
    fn new(sentence: String) -> Self {
        Self { sentence }
    }

    fn find_words(&self) {
        let words: Vec<&str> = self.sentence.trim().split_whitespace().collect();

        if words.is_empty() {
            println!("No words");
            return;
        }

        let mut longest = words[0];
        let mut shortest = words[0];

        for &word in &words {
            if word.len() > longest.len() {
                longest = word;
            }
            if word.len() < shortest.len() {
                shortest = word;
            }
        }

        println!("Longest word: {}", longest);
        println!("Shortest word: {}", shortest);
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter a sentence:");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let analyzer = SentenceAnalyzer::new(input);
    analyzer.find_words();
}
