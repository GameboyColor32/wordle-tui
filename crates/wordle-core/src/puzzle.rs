use crate::Word;
use crate::CoreError;

#[derive(Debug)]
pub struct Puzzle {
    id: u64,
    solution: Word,
}

impl Puzzle {
    pub fn new(id: u64, solution: &str) -> Result<Self, CoreError> {
        Ok(Self {
            id,
            solution: Word::new(solution)?,
        })
    }
}

