use super::words::is_valid_guess;
use colored::Colorize;

#[derive(Clone)]
pub struct Guess {
    guess: String,
    word: &'static str,
}

impl Guess {
    pub fn new(guess: &str, word: &'static str) -> Result<Self, &'static str> {
        if guess.len() != 5 {
            Err("Guess must be 5 characters long")
        } else if !is_valid_guess(guess) {
            Err("Invalid guess")
        } else {
            Ok(Self {
                guess: String::from(guess),
                word,
            })
        }
    }

    pub fn print(&self) {
        let guess: String = self.clone().into();
        println!("{}", guess);
    }

    pub fn is_correct(&self) -> bool {
        self.guess.eq(self.word)
    }
}

impl Into<String> for Guess {
    fn into(self) -> String {
        let mut result = String::new();
        let mut word = String::from(self.word);

        for (i, c) in self.guess.chars().enumerate() {
            if c == word.chars().nth(i).unwrap() {
                result.push_str(&c.to_string().on_bright_green().to_string());
            } else if word.chars().any(|x| x == c) {
                result.push_str(&c.to_string().black().on_yellow().to_string());
            } else {
                result.push_str(&c.to_string());
            }
            // This is kinda ugly
            word = word.replacen(c, "_", 1);
        }

        result
    }
}
