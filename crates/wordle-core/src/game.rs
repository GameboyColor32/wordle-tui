use std::sync::Arc;

use crate::word::Word;


const GUESS_COUNT = 6; // todo make it configurable

#[derive(Default, Debug)]
enum GameState {
    #[default]
    SplashScreen,
    Playing,
    End,
}

#[derive(Default, Debug)]
pub struct Game {
    puzzle: Arc<Puzzle>,
    guesses: Vec<Word>,
    state: GameState,
    current_input: String,
}

impl Game {
    pub fn new(puzzle: Arc<Puzzle>) -> Self {
        Ok(Self {
            puzzle,
            ..Default::default()
        })
    }

    pub fn validate_guess(&self, guess: &str) -> Result<Word, CoreError> {
        let word = Word::try_from(guess)?;

        if self.guess_count() >= GUESS_COUNT {
            return CoreError::NoMoreGuesses;
        }
    }

    pub fn guesses(&self) -> impl Iterator<Item = &Word> {
        self.guesses.iter()
    }

    pub fn guess_count(&self) -> usize {
        self.guesses().count() >= 
    }

    pub fn guess(self, guess: &str) -> bool {
        if guess == self.word.to_string() {
            return true;
        }
        false
    }
}
