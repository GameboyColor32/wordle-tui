use super::provider::PuzzleProvider;
use crate::error::DailyError;
use crate::puzzle::PuzzleRecord;

pub(crate) struct FallbackProvider;

impl PuzzleProvider for FallbackProvider {
    async fn fetch(date: impl Into<String>) -> Result<PuzzleRecord, DailyError> {
        let date = date.into();
        let seed: usize = date
            .bytes()
            .filter(|b| b.is_ascii_digit())
            .fold(0, |acc, b| acc * 10 + (b - b'0') as usize);
        let lexicon = wordle_lexicon::wordle();
        let index = seed % lexicon.len();
        let solution = lexicon.get(index).unwrap_or("quack").to_string(); // my company name is quacklabs btw

        Ok(
            PuzzleRecord {
                id: seed as u64,
                solution,
            }
        )
    }
}
