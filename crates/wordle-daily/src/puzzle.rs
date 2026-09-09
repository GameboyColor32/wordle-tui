use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PuzzleRecord {
    pub id: u64,
    pub solution: String,
}
