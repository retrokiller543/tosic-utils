use crate::db::query::traits::Statement;

pub struct Update;

impl Statement for Update {
    const STATEMENT: &'static str = "UPDATE";
}
