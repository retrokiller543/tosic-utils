use thiserror::Error;
use crate::prelude::DatabaseError;
#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),
}