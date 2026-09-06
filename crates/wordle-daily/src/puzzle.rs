use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Puzzle {
    pub id: u64,
    pub solution: String,
}
