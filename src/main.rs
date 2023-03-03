pub mod wordle;

use colored::Colorize;
use wordle::game::*;

fn main() {
    println!("Wordle in Rust");

    let word = gen_random_word();

    let mut count = 1u32;

    loop {
        println!("Enter a guess: ");

        let mut guess = String::with_capacity(5);

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match Guess::new(guess.trim(), word) {
            Ok(guess) => guess,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        // delete previous line
        print!("\x1b[1A\x1b[2K");

        let colored_guess: String = guess.clone().into();

        println!("{}", colored_guess);

        if guess.is_correct() {
            println!(
                "{} {} {}",
                "You won after".green(),
                count.to_string().green(),
                if count == 1 { "guess!" } else { "guesses!" }
                    .to_string()
                    .green()
            );
            break;
        }

        count += 1;
    }
}
