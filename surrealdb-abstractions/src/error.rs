use crate::prelude::DatabaseError;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),
}
