use super::error::DatabaseError;
use crate::db::query::traits::{Query, Statement};
use crate::db::QueryBuilder;
use crate::{InternalResult, Result};
use log::error;
use once_cell::sync::Lazy;
use std::sync::Arc;
use surrealdb::opt::QueryResult;
use surrealdb::{Connection, Surreal};

pub(crate) struct Runner;

impl Runner {
    pub fn new() -> Self {
        Self
    }

    fn handle_response<U>(
        mut res: surrealdb::Response,
        index: impl QueryResult<U>,
    ) -> InternalResult<U>
    where
        U: serde::de::DeserializeOwned,
    {
        let data: U = match res.take(index) {
            Ok(data) => data,
            Err(err) => {
                error!("Error running query: {}", err);
                return Err(DatabaseError::ResponseError(err));
            }
        };

        Ok(data)
    }

    pub(crate) async fn run<C, U, Q>(
        db: &Arc<Surreal<C>>,
        query: Q,
        index: impl QueryResult<U>,
    ) -> InternalResult<U>
    where
        C: Connection,
        U: serde::de::DeserializeOwned,
        Q: Query,
    {
        let query = query.construct();

        let res = match db.query(&query).await {
            Ok(res) => res,
            Err(err) => {
                error!("Error running query: {} with error: {}", query, err);
                return Err(DatabaseError::TransactionError(err));
            }
        };

        Self::handle_response(res, index)
    }

    pub(crate) async fn run_lazy<C, U, Q>(
        db: &Lazy<Surreal<C>>,
        query: Q,
        index: impl QueryResult<U>,
    ) -> InternalResult<U>
    where
        C: Connection,
        U: serde::de::DeserializeOwned,
        Q: Query,
    {
        let query = query.construct();

        let res = match db.query(&query).await {
            Ok(res) => res,
            Err(err) => {
                error!("Error running query: {} with error: {}", query, err);
                return Err(DatabaseError::TransactionError(err));
            }
        };

        Self::handle_response(res, index)
    }
}

impl<Type> QueryBuilder<Type>
where
    Self: Query,
    Type: Statement + ?Sized,
{
    pub async fn run<C, U>(self, db: &Arc<Surreal<C>>, index: impl QueryResult<U>) -> Result<U>
    where
        C: Connection,
        U: serde::de::DeserializeOwned,
    {
        Runner::run(db, self, index).await.map_err(|e| e.into())
    }

    pub async fn run_lazy<C, U>(
        self,
        db: &Lazy<Surreal<C>>,
        index: impl QueryResult<U>,
    ) -> Result<U>
    where
        C: Connection,
        U: serde::de::DeserializeOwned,
    {
        Runner::run_lazy(db, self, index)
            .await
            .map_err(|e| e.into())
    }
}
