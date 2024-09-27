use crate::db::query::traits::Statement;
use crate::db::QueryBuilder;
use crate::prelude::Query;

pub struct Insert;

impl Statement for Insert {
    const STATEMENT: &'static str = "INSERT";
}

impl Query for QueryBuilder<Insert> {
    fn construct(self) -> String {
        todo!("This statement is not yet implemented!")
    }
}
