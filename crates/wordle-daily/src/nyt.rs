use reqwest;

use crate::puzzle::Puzzle;
use crate::error::DailyError;

static nyt_url: &'static str = "https://www.nytimes.com/svc/wordle/v2/";

pub(crate) async fn fetch_nyt(date: impl Into<String>) -> Result<Puzzle, DailyError> {
    let date = date.into();
    let url = format!("{nyt_url}{date}.json");

    Ok(reqwest::get(url)
        .await?
        .error_for_status()?
        .json::<Puzzle>()
        .await?)
}
