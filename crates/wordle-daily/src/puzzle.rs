use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Puzzle {
    pub id: u64,
    pub solution: String,
}
