use thiserror::Error;

pub type CorpusResult<T> = Result<T, CorpusError>;

#[derive(Error, Debug)]
pub enum CorpusError {
    #[error("Error accessing backing storage")]
    BackingStorageError(#[from] std::io::Error),
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    #[error("Error decoding {0}")]
    DecodingError(String),
    #[error("Error encoding {0}")]
    EncodingError(String),
    #[error("Lock error: {0}")]
    LockError(String),
    #[error("{0} id overflow")]
    IdOverflowError(String),
    #[error("String not found between {0} and {1}")]
    StringNotFoundError(u64, u64),
    #[error("Invalid string found between {0} and {1}")]
    InvalidStringError(usize, usize),
}
