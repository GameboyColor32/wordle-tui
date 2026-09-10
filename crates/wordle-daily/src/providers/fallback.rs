use super::provider::PuzzleProvider;
use crate::error::DailyError;
use crate::puzzle::PuzzleRecord;

const WORDS: &str = include_str!("../../data/words.txt");

pub(crate) struct FallbackProvider;

impl PuzzleProvider for FallbackProvider {
    async fn fetch(date: impl Into<String>) -> Result<PuzzleRecord, DailyError> {
        let date = date.into();
        let seed: usize = date
            .bytes()
            .filter(|b| b.is_ascii_digit())
            .fold(0, |acc, b| acc * 10 + (b - b'0') as usize);
        let word_count = WORDS.lines().count();
        let solution = seed
            .checked_rem(word_count)
            .and_then(|index| WORDS.lines().nth(index))
            .unwrap_or("quack") // my company name is quacklabs btw
            .to_owned();

        Ok(
            PuzzleRecord {
                id: seed as u64,
                solution,
            }
        )
    }
}
