#[derive(Debug, thiserror::Error)]
pub enum DailyError {
    #[error("failed to fetch puzzle")]
    Request(#[from] reqwest::Error),

    #[error("failed to access puzzle cache")]
    Io(#[from] std::io::Error),
}
