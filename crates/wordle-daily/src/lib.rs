mod error;
mod nyt;
mod puzzle;

use std::path::PathBuf;

use chrono::NaiveDate;

use crate::puzzle::Puzzle;
use crate::error::DailyError;
use crate::nyt::fetch_nyt;

pub struct Cache {
    directory: PathBuf,
}

impl Cache {
    pub fn new(directory: impl Into<PathBuf>) -> Self {
        Self {
            directory: directory.into(),
        }
    }

    fn path_for(&self, date: NaiveDate) -> PathBuf {
        self.directory.join(format!("{}.json", date.format("%Y-%m-%d")))
    }

    pub async fn load(&self, date: NaiveDate) -> Result<Option<Puzzle>, DailyError> {
        let path = self.path_for(date);

        println!("{}", path.display());
        if !path.exists() {
            let word = fetch_nyt(date.format("%Y-%m-%d").to_string()).await?;

            println!("{:?}", word);
        }
        todo!("fetch word of the day")
    }
}

/*
pub struct DailyWordle {
    word: Arc<>
}
*/
