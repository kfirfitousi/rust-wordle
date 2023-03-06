mod wordle;
mod words;

use std::env::args;
use wordle::Wordle;

fn main() {
    let max_tries = if args().nth(1).unwrap_or_default().eq("--easy") {
        u32::MAX
    } else {
        6
    };

    let mut wordle = Wordle::new(max_tries);

    wordle.print_welcome();

    loop {
        wordle.print_prompt();

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // delete input line
        print!("\x1b[1A\x1b[2K\x1b[1A\x1b[2K");

        let guess = match wordle.make_guess(guess.trim()) {
            Ok(guess) => guess,
            Err(_) => continue,
        };

        guess.print();

        if guess.is_correct() {
            wordle.print_win();
            break;
        }

        if wordle.is_game_over() {
            wordle.print_game_over();
            break;
        }
    }
}
