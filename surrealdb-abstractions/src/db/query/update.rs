use crate::db::query::query_parts::{content, filter, parallel, timeout};
use crate::db::query::traits::Statement;
use crate::db::query::ContentType;
use crate::db::{Query, QueryBuilder};
use crate::{impl_content, impl_filter, impl_only, impl_parallel, impl_timeout};

pub struct Update;

impl Statement for Update {
    const STATEMENT: &'static str = "UPDATE";
}

impl QueryBuilder<Update> {
    impl_content!();
    impl_only!();
    impl_parallel!();
    impl_filter!();
    impl_timeout!();
}

impl Query for QueryBuilder<Update> {
    #[inline]
    fn construct(self) -> String {
        let mut query = String::new();

        query.push_str(Self::STATEMENT);

        if self.only {
            query.push_str(" ONLY");
        }

        query.push_str(" ");
        query.push_str(&self.table);

        if self.content.is_none() {
            panic!("Content must be specified!");
        }

        content(ContentType::Merge, self.content, &mut query);

        filter(self.filter, &mut query);

        timeout(self.timeout, &mut query);

        parallel(self.parallel, &mut query);

        query
    }
}
