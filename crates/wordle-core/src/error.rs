#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("word must contain exactly letters, but contains {actual}")]
    InvalidWordLength { actual: usize },


    #[error("word must contain only ASCII letters")]
    InvalidCharacter,
}
