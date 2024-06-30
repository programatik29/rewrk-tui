use thiserror::Error;

pub type BenchResult<T> = Result<T, BenchError>;

#[derive(Debug, Error)]
pub enum BenchError {
    #[error("failed to connect")]
    ConnFailed,
    #[error("connection closed")]
    ConnClosed,
}

impl BenchError {
    pub fn conn_failed<E>(_e: E) -> Self {
        Self::ConnFailed
    }

    pub fn conn_closed<E>(_e: E) -> Self {
        Self::ConnClosed
    }
}
