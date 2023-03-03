pub mod game {
    use colored::Colorize;
    use rand::Rng;

    static WORDS: &'static [&'static str] = &[
        "about", "after", "again", "below", "could", "every", "first", "found", "great", "house",
        "large", "learn", "never", "other", "place", "plant", "point", "right", "small", "sound",
        "spell", "still", "study", "their", "there", "these", "thing", "think", "three", "water",
        "where", "which", "world", "would", "write", "apple", "bible", "black", "board", "bread",
        "broke", "brown", "build", "bunch", "catch", "chair", "chick", "chief", "child", "church",
        "clean", "clock", "cloud", "coast", "color", "could", "count", "cover", "cream", "cross",
        "dance", "death", "dirty", "dress", "drink", "drive", "earth", "eight", "enter", "equal",
        "every", "exact", "extra", "faith", "false", "field", "fight", "finger", "first", "floor",
        "flower", "fruit", "glass", "glove", "goose", "grass", "green", "group", "guess", "happy",
        "heart", "heavy", "horse", "house", "human", "image", "jelly", "jewel", "judge", "knife",
    ];

    pub fn gen_random_word() -> &'static str {
        let index = rand::thread_rng().gen_range(0..WORDS.len());
        WORDS[index]
    }

    #[derive(Clone)]
    pub struct Guess {
        pub guess: String,
        word: &'static str,
    }

    impl Guess {
        pub fn new(guess: &str, word: &'static str) -> Result<Self, &'static str> {
            if guess.len() != 5 {
                Err("Guess must be 5 characters long")
            } else {
                Ok(Self {
                    guess: String::from(guess),
                    word,
                })
            }
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
                    result.push_str(&c.to_string().on_green().to_string());
                } else if word.chars().any(|x| x == c) {
                    result.push_str(&c.to_string().on_yellow().to_string());
                } else {
                    result.push_str(&c.to_string());
                }
                // This is kinda ugly
                word = word.replacen(c, "_", 1);
            }

            result
        }
    }
}
