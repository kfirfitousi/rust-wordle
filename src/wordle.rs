use crate::words::{gen_random_word, is_valid_guess};
use colored::Colorize;

pub struct Wordle {
    word: &'static str,
    tries: u32,
    max_tries: u32,
}

impl Wordle {
    /// Creates a new Wordle game.
    pub fn new(max_tries: u32) -> Self {
        Self {
            word: gen_random_word(),
            tries: 0,
            max_tries,
        }
    }
    /// Makes a guess.
    /// Returns an error if the guess is invalid.
    pub fn make_guess(&mut self, guess: &str) -> Result<Guess, ()> {
        let guess = Guess::new(guess, self.word)?;
        self.tries += 1;
        Ok(guess)
    }
    /// Returns true if the player has no more tries left.
    pub fn is_game_over(&self) -> bool {
        self.tries >= self.max_tries
    }
    /// Prints the welcome message.
    pub fn print_welcome(&self) {
        println!("{}", "Welcome to Wordle in Rust!".bright_white().bold(),);
    }
    /// Prints the prompt.
    pub fn print_prompt(&self) {
        println!(
            "{} {}",
            "Guess the word:".bright_white().bold(),
            format!("({} tries left)", self.max_tries - self.tries).bright_white()
        );
    }
    /// Prints the game over message.
    pub fn print_game_over(&self) {
        println!(
            "{} {}",
            "You lost! The word was".bright_red().bold(),
            self.word.bold()
        );
    }
    /// Prints the win message.
    pub fn print_win(&self) {
        println!(
            "{} {} {}",
            "You won after".bright_green().bold(),
            self.tries.to_string().bright_green().bold(),
            if self.tries == 1 { "try!" } else { "tries!" }
                .to_string()
                .bright_green()
                .bold()
        );
    }
}

#[derive(Clone)]
pub struct Guess {
    guess: String,
    word: &'static str,
}

impl Guess {
    /// Creates a new guess.
    /// Returns an error if the guess or the word is invalid.
    fn new(guess: &str, word: &'static str) -> Result<Self, ()> {
        if guess.len() != 5 {
            print!("{}", "Guess must be 5 characters long. ".bright_red());
            Err(())
        } else if !is_valid_guess(guess) {
            print!("{}", "Invalid guess. ".bright_red());
            Err(())
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
    /// Converts a guess into a colored string.
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
