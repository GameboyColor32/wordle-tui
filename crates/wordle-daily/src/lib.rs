mod error;
mod nyt;
mod provider;
mod puzzle;
mod fallback;

use std::path::PathBuf;

use chrono::NaiveDate;
use tokio::fs::{create_dir_all, try_exists, write, read};

use crate::puzzle::PuzzleRecord;
use crate::fallback::FallbackProvider;
use crate::provider::PuzzleProvider;
use crate::error::DailyError;
use crate::nyt::NytProvider;

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

    pub async fn load(&self, date: NaiveDate) -> Result<PuzzleRecord, DailyError> {
        let path = self.path_for(date);

        if !try_exists(&self.directory).await? {
            println!("INFO: {} doesn't exist, creating...", &self.directory.display());
            create_dir_all(&self.directory).await?;
        }
        if !try_exists(&path).await? {
            let str_date = date.format("%Y-%m-%d").to_string();

            let puzzle = match NytProvider::fetch(str_date.clone()).await {
                Ok(puzzle) => puzzle,
                Err(err) => {
                    eprintln!("ERROR {err:?}: using fallback...",);
                    FallbackProvider::fetch(str_date).await?
                }
            };
            let json = serde_json::to_vec_pretty(&puzzle)?;
            write(&path, json).await?;
            println!("INFO: daily wordle saved at {}", path.display());
            return Ok(puzzle);
        }
        let bytes = read(&path).await?;
        let puzzle = serde_json::from_slice::<PuzzleRecord>(&bytes)?;
        println!("INFO: daily wordle read from {}", path.display());

        Ok(puzzle)
    }
}

