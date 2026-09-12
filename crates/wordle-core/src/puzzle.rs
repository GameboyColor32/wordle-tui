use crate::Word;

pub struct Puzzle {
    id: u64,
    solution: Word,
}

impl Puzzle {
    pub fn new(id: u64, solution: impl Into<String>) -> Result<Self, CoreError> {
        Self {
            id,
            solution: Word::new(solution)?,
        }
    }
}
