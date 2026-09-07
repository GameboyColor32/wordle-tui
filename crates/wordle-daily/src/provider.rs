use crate::error::DailyError;
use crate::puzzle::Puzzle;

pub(crate) trait PuzzleProvider {
    async fn fetch(date: impl Into<String>) -> Result<Puzzle, DailyError>;
}
