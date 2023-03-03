pub mod wordle;

use colored::Colorize;
use wordle::game::Guess;
use wordle::words::gen_random_word;

fn main() {
    println!("Wordle in Rust");

    let word = gen_random_word();

    let mut tries = 1u32;

    loop {
        println!("Enter a guess: ");

        let mut guess = String::with_capacity(5);

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // delete 2 previous lines
        print!("\x1b[1A\x1b[2K\x1b[1A\x1b[2K");

        let guess = match Guess::new(guess.trim(), word) {
            Ok(guess) => guess,
            Err(e) => {
                println!("{}", e.black().on_yellow());
                continue;
            }
        };

        guess.print();

        if guess.is_correct() {
            println!(
                "{} {} {}",
                "You won after".bright_green(),
                tries.to_string().bright_green(),
                if tries == 1 { "guess!" } else { "guesses!" }
                    .to_string()
                    .bright_green()
            );
            break;
        }

        tries += 1;
    }
}
