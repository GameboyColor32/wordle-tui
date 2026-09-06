use std::path::PathBuf;
use chrono::NaiveDate;

pub struct Cache {
    pub directory: PathBuf,
}

impl Cache {
    pub async fn load(&self, date: NaiveDate) { // -> Result<Option<Puzzle>, DailyError> {
        let formatted_date = date.format("%Y-%m-%d");
        println!("{}", formatted_date);

        if self.directory.push(format!("{}.json", formatted_date)).exists() {
            todo!("return game (inverse this condition next tine)")
        }
        todo!("fetch word of the day")
    }
}

/*
pub struct DailyWordle {
    word: Arc<>
}
*/

