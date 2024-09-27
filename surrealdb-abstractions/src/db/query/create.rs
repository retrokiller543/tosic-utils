use crate::db::query::traits::{Query, Statement};
use crate::db::query::ContentType;
use crate::db::QueryBuilder;
use crate::prelude::query::query_parts::content;
use crate::{impl_content, impl_only, impl_parallel, impl_timeout};
use log::debug;

pub struct Create;

impl Statement for Create {
    const STATEMENT: &'static str = "CREATE";
}

impl QueryBuilder<Create> {
    impl_only!();
    impl_parallel!();
    impl_timeout!();
    impl_content!();
}

impl Query for QueryBuilder<Create> {
    fn construct(self) -> String {
        let mut query = String::new();
        query.push_str(Self::STATEMENT);

        if self.only {
            query.push_str(" ONLY");
        }

        query.push_str(format!(" {}", self.table).as_str());

        content(ContentType::Content, self.content, &mut query);

        if let Some((duration, unit)) = self.timeout {
            query.push_str(format!(" TIMEOUT {}{}", duration, unit).as_str());
        }

        if self.parallel {
            query.push_str(" PARALLEL");
        }

        debug!("Constructed query: {}", query);

        query
    }
}
