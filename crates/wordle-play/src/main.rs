use chrono::Local;

use wordle_daily::Cache;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let date = Local::now().date_naive();
    let cache = Cache::new("./cache");

    let puzzle = cache.load(date).await;
    Ok(())
}
