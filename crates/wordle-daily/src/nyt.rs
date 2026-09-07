use reqwest;

use crate::provider::PuzzleProvider;
use crate::error::DailyError;
use crate::puzzle::Puzzle;

static NYT_URL: &'static str = "https://www.nytimes.com/svc/wordle/v2/";

pub(crate) struct NytProvider;

impl PuzzleProvider for NytProvider {
    async fn fetch(date: impl Into<String>) -> Result<Puzzle, DailyError> {
        let date = date.into();
        let url = format!("{NYT_URL}{date}.json");

        Ok(reqwest::get(url)
            .await?
            .error_for_status()?
            .json::<Puzzle>()
            .await?)
    }
}
