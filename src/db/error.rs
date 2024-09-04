use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Response error: {0}. Possibly the wrong type of response?")]
    ResponseError(#[from] surrealdb::Error),

    #[error("Transaction error: {0}")]
    TransactionError(surrealdb::Error),

    #[error("Database error: {0}")]
    Error(#[from] anyhow::Error),
}
