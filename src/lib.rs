#![feature(type_alias_impl_trait)]
#![feature(extern_types)]

use once_cell::sync::Lazy;
use surrealdb::engine::local::Db;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::db::DatabaseError;
use crate::error::Error;

pub mod db;
pub mod error;
pub mod prelude;


pub mod ffi;

pub type Result<T> = std::result::Result<T, Error>;
pub(crate) type InternalResult<T> = std::result::Result<T, DatabaseError>;

#[cfg(test)]
pub(crate) static TEST_DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[cfg(test)]
mod test {
    use surrealdb::engine::remote::ws::Ws;
    use super::*;

    pub async fn init_db() -> anyhow::Result<()> {
        TEST_DB.connect::<Ws>("localhost:8234").await?;
        TEST_DB.use_ns("test").use_db("test").await?;

        Ok(())
    }
}
