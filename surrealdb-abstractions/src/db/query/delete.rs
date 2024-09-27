use crate::db::query::traits::{Query, Statement};
use crate::db::QueryBuilder;
use crate::{impl_filter, impl_only, impl_parallel};
use log::debug;

pub struct Delete;

impl Statement for Delete {
    const STATEMENT: &'static str = "DELETE";
}

impl QueryBuilder<Delete> {
    impl_filter!();
    impl_only!();
    impl_parallel!();
}

impl Query for QueryBuilder<Delete> {
    fn construct(self) -> String {
        let mut query = String::new();
        query.push_str(Self::STATEMENT);
        query.push(' ');

        if self.only {
            query.push_str("ONLY ");
        }

        query.push_str(&self.table);

        query.push(' ');
        query.push_str(&self.filter.construct());

        if self.only {
            query.push_str(" RETURN $before");
        }

        if self.parallel {
            query.push_str(" PARALLEL");
        }

        debug!("Constructed query: {}", query);

        query
    }
}
