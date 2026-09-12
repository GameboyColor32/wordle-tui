use std::sync::Arc;

use crate::word::Word;
use crate::puzzle::Puzzle;
use crate::error::CoreError;


const GUESS_COUNT: usize = 6; // todo make it configurable

#[derive(Debug)]
enum GameState {
    SplashScreen,
    Playing,
    End,
}

#[derive(Debug)]
pub struct Game {
    puzzle: Arc<Puzzle>,
    guesses: Vec<Word>,
    state: GameState,
    current_input: String,
}

impl Game {
    pub fn new(puzzle: Arc<Puzzle>) -> Self {
        Self {
            puzzle,
            guesses: Vec::new(),
            state: GameState::SplashScreen, // todo remake splashscreen
            current_input: String::new(),
        }
    }

    fn is_valid(guess: &str) -> bool {
        wordle_lexicon::wordle().contains(guess)
    }

    pub fn validate_guess(&self, guess: &str) -> Result<Word, CoreError> {
        let word = Word::try_from(guess)?;

        if self.guess_count() >= GUESS_COUNT {
            return Err(CoreError::NoMoreGuesses);
        }
        if !Self::is_valid(guess) {
            return Err(CoreError::InvalidWord);
        }
        Ok(word)
    }

    pub fn guesses(&self) -> impl Iterator<Item = &Word> {
        self.guesses.iter()
    }

    pub fn guess_count(&self) -> usize {
        self.guesses().count()
    }
}
