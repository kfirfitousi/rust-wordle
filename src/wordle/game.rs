use super::words::is_valid_guess;
use colored::Colorize;

#[derive(Clone)]
pub struct Guess {
    guess: String,
    word: &'static str,
}

impl Guess {
    /// Creates a new guess.
    /// Returns an error if the guess or the word is invalid.
    pub fn new(guess: &str, word: &'static str) -> Result<Self, &'static str> {
        if word.len() != 5 {
            Err("Word must be 5 characters long.")
        } else if guess.len() != 5 {
            Err("Guess must be 5 characters long.")
        } else if !is_valid_guess(guess) {
            Err("Invalid guess.")
        } else {
            Ok(Self {
                guess: String::from(guess),
                word,
            })
        }
    }

    /// Prints the guess with colored letters.    
    /// Correctly positioned letters are green, letters in wrong position are yellow.
    pub fn print(&self) {
        let guess: String = self.clone().into();
        println!("{}", guess);
    }

    /// Returns true if the guess is correct.
    pub fn is_correct(&self) -> bool {
        self.guess.eq(self.word)
    }
}

impl Into<String> for Guess {
    fn into(self) -> String {
        let mut result = vec![String::new(); 5];
        let mut word_chars = self.word.chars().collect::<Vec<char>>();
        let guess_chars = self.guess.chars().collect::<Vec<char>>();

        // first iteration - color correctly positioned letters
        for (i, c) in guess_chars.iter().enumerate() {
            if c == &word_chars[i] {
                result[i] = c.to_string().bold().on_bright_green().to_string();
                word_chars[i] = '_';
            } else {
                result[i] = c.to_string().bold().to_string();
            }
        }

        // second iteration - color letters in wrong position
        for (i, c) in guess_chars.iter().enumerate() {
            if word_chars[i] == '_' {
                continue;
            }
            if let Some(index) = word_chars.iter().position(|x| x == c) {
                if word_chars[index] == '*' {
                    continue;
                }
                result[i] = c.to_string().bold().on_yellow().to_string();
                word_chars[index] = '*';
            }
        }

        result.concat()
    }
}
