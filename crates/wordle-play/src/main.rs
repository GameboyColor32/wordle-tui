use std::sync::Arc;

use chrono::Local;

use wordle_daily::Cache;
use wordle_core::{Game, Puzzle, Word};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let date = Local::now().date_naive();
    let cache = Cache::new("./cache");

    let record = cache.load(date).await?; // todo return errors
    let puzzle = Arc::new(Puzzle::new(record.id, &record.solution)?);
    let game = Game::new(puzzle);

    let guess = game.validate_guess("apple");

    let test_b = Word::new("alley").unwrap();
    let test_a = Word::new("apple").unwrap();

    let result = test_a.evaluate_agains(test_b);

    println!("result a {:?}", result);
    let result = test_b.evaluate_agains(test_a);

    println!("result b {:?}", result);
    // todo wrap game in app and run loop
    Ok(())
}
