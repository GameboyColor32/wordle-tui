use std::fmt;

use crate::error::CoreError;

const WORD_LENGTH: usize = 5;

#[derive(Debug, Eq, Copy, Clone, PartialEq)]
pub enum LetterState {
    Correct,
    Present,
    Absent,
}

#[derive(Debug, Eq, Copy, Clone, PartialEq)]
pub struct LetterResult {
    pub letter: char,
    pub state: LetterState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Word {
    letters: [u8; WORD_LENGTH]
}

impl Word {
    pub fn new(word: &str) -> Result<Self, CoreError> {
        word.try_into()
    }

    pub fn evaluate_agains(&self, solution: Self) -> [LetterResult; WORD_LENGTH] {
        let mut states = [LetterState::Absent; WORD_LENGTH];
        let mut remaining = [0_u8; 26];

        for index in 0..WORD_LENGTH {
            if self.letters[index] == solution.letters[index] {
                states[index] = LetterState::Correct;
            } else {
                remaining[letter_index(solution.letters[index])] += 1;
            }
        }

        for index in 0..WORD_LENGTH {
            if states[index] == LetterState::Correct {
                continue;
            }

            let letter = letter_index(self.letters[index]);

            if remaining[letter] > 0 {
                states[index] = LetterState::Present;
                remaining[letter] -= 1;
            }
        }

        std::array::from_fn(|index| LetterResult {
            letter: self.letters[index] as char,
            state: states[index],
        })
    }
}

impl TryFrom<&str> for Word {
    type Error = CoreError;

    fn try_from(word: &str) -> Result<Self, Self::Error> {
        if word.len() != WORD_LENGTH {
            return Err(CoreError::InvalidWordLength { actual: word.chars().count() });
        }
        if !word.bytes().all(|byte| byte.is_ascii_alphabetic()) {
            return Err(CoreError::InvalidCharacter);
        }

        let mut letters = [0; WORD_LENGTH];
        for (destination, letter) in letters.iter_mut().zip(word.bytes()) {
            *destination = letter.to_ascii_uppercase();
        }
        Ok(Self { letters })
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word = std::str::from_utf8(&self.letters).expect("Word should always contain ASCII letters");

        f.write_str(word)
    }
}

fn letter_index(letter: u8) -> usize {
    usize::from(letter - b'A')
}

