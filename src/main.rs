pub mod wordle;

use colored::Colorize;
use wordle::game::Guess;
use wordle::words::gen_random_word;

fn main() {
    println!("{}", "Wordle in Rust".bright_white().bold());

    let word = gen_random_word();

    let mut tries = 0;

    let infinite_tries = std::env::args()
        .nth(1)
        .eq(&Some(String::from("--infinite-tries")));

    loop {
        println!("{}", "Enter a guess: ".bright_white());

        let mut guess = String::with_capacity(5);

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // delete input line
        print!("\x1b[1A\x1b[2K\x1b[1A\x1b[2K");

        let guess = match Guess::new(guess.trim(), word) {
            Ok(guess) => guess,
            Err(e) => {
                print!("{} ", e.bright_red());
                continue;
            }
        };

        tries += 1;
        guess.print();

        if guess.is_correct() {
            println!(
                "{} {} {}",
                "You won after".bright_green().bold(),
                tries.to_string().bright_green().bold(),
                if tries == 1 { "guess!" } else { "guesses!" }
                    .to_string()
                    .bright_green()
                    .bold()
            );
            break;
        }

        if !infinite_tries && tries == 6 {
            println!(
                "{} {}",
                "You lost! The word was".bright_red().bold(),
                word.bold()
            );
            break;
        }
    }
}
