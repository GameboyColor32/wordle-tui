mod word;

pub use word::Word;

#[derive(Default, Debug)]
enum GameState {
    #[default]
    SplashScreen,
    Playing,
    End,
}

#[derive(Default, Debug)]
pub struct Game {
    word: Word,
    guesses: [Word; 6],
    state: GameState,
    current_input: String,
}

impl Game {
    pub fn new(word: impl Into<String>) -> Result<Self, &'static str> {
        let word = Word::new(word)?;

        Ok(Self {
            word,
            ..Default::default()
        })
    }

    pub fn guess(self, guess: &str) -> bool {
        if guess == self.word.to_string() {
            return true;
        }
        false
    }
}
