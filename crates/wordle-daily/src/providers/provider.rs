use crate::error::DailyError;
use crate::puzzle::PuzzleRecord;

pub(crate) trait PuzzleProvider {
    async fn fetch(date: impl Into<String>) -> Result<PuzzleRecord, DailyError>;
}
