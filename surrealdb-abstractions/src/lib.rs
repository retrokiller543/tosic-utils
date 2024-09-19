#![feature(type_alias_impl_trait)]
#![feature(extern_types)]

use crate::db::DatabaseError;
use crate::error::Error;

pub mod db;
pub mod error;
pub mod prelude;

pub type Result<T> = std::result::Result<T, Error>;
pub(crate) type InternalResult<T> = std::result::Result<T, DatabaseError>;

#[cfg(test)]
mod test {
    use super::*;
    use once_cell::sync::Lazy;
    use surrealdb::engine::local::{Db, Mem};
    use surrealdb::engine::remote::ws::Client;
    use surrealdb::engine::remote::ws::Ws;
    use surrealdb::Surreal;

    pub(crate) static TEST_DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

    pub async fn init_db() -> anyhow::Result<()> {
        TEST_DB.connect::<Ws>("localhost:8234").await?;
        TEST_DB.use_ns("test").use_db("test").await?;

        Ok(())
    }

    pub async fn db() -> anyhow::Result<Surreal<Db>> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("test").use_db("test").await?;

        Ok(db)
    }

    pub fn init_tracing() {
        let subscriber = tracing_subscriber::fmt()
            .with_file(true)
            .with_line_number(true)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }
}
