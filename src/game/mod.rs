pub struct Word {
    letters: [String; 5]
}

impl Word {
    pub fn new(word: impl Into<String>) -> Result<Self, &'static str> {
        let word = word.into();

        if word.chars().count() != 5 {
            return Err("String must be less than 5 chars");
        }
        let is_word = word.chars().all(|c| c.is_ascii_alphabetic());
        println!("{}", is_word);

        if word.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err("String must be alphabetic");
        }
        Ok(Self{letters: std::array::from_fn(|i| {
            word.chars().nth(i).unwrap().to_string()
        })})
    }
}

#[derive(Default, Debug)]
enum GameState {
    #[default] SplashScreen,
    Playing,
    End
}

#[derive(Default, Debug)]
pub struct Game {
    word: String,
    guesses: [String; 6],
    state: GameState,
    current_input: String,
}

impl Game {
    pub fn new(word: impl Into<String>) -> Result<Self, &'static str> {
        let word = word.into();

        if word.chars().count() != 5 {
            return Err("String must be less than 5 chars");
        }

        Ok(Self {
            word, ..Default::default()
        })
    }

    pub fn guess(self, guess: &str) -> bool {
        if guess == self.word {
            return true;
        }
        false
    } 
}
