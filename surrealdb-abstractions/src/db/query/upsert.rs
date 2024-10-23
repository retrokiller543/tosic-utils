use crate::db::query::query_parts::{content, filter, parallel, timeout};
use crate::db::query::traits::Statement;
use crate::db::query::ContentType;
use crate::db::QueryBuilder;
use crate::prelude::Query;
use crate::{impl_content, impl_filter, impl_only, impl_parallel, impl_timeout};

pub struct Upsert;

impl Statement for Upsert {
    const STATEMENT: &'static str = "UPSERT";
}

impl Query for QueryBuilder<Upsert> {
    fn construct(self) -> String {
        let mut query = String::new();
        query.push_str(Self::STATEMENT);
        query.push(' ');

        if self.only {
            query.push_str("ONLY ");
        }

        query.push_str(&self.table);

        query.push(' ');

        if self.content.is_none() {
            panic!("Content must be specified!")
        }

        content(ContentType::Content, self.content, &mut query);

        filter(self.filter, &mut query);

        timeout(self.timeout, &mut query);

        parallel(self.parallel, &mut query);

        query
    }
}

impl QueryBuilder<Upsert> {
    impl_only!();
    impl_parallel!();
    impl_timeout!();
    impl_content!();
    impl_filter!();
}
