#[derive(Debug, thiserror::Error, PartialEq)]
pub enum TokenizeError {
    #[error("invalid character: {0}")]
    InvalidOperator(char),
    #[error("invalid number: {0}")]
    InvalidNumber(String),
    #[error("Failed to tokenize at: {0}")]
    InvalidSyntax(String),
}

pub type TokenizeResult<T, E = TokenizeError> = anyhow::Result<T, E>;
