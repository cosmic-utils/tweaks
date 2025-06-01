use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("RON error: {0}")]
    Ron(#[from] ron::error::SpannedError),
    #[error("Iced error: {0}")]
    Iced(#[from] cosmic::iced::Error),
    #[error("Theme path not found")]
    ThemePathNotFound,
    #[error("Layout path not found")]
    LayoutPathNotFound,
}
