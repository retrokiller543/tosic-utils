use crate::db::query::query_parts::content;
use crate::prelude::query::ContentType;
use crate::prelude::*;
use crate::{impl_content, impl_only, impl_parallel, impl_relation, impl_timeout};
use log::debug;

pub struct Relate;

impl Statement for Relate {
    const STATEMENT: &'static str = "RELATE";
}

impl QueryBuilder<Relate> {
    impl_timeout!();
    impl_only!();
    impl_parallel!();
    impl_content!();
    impl_relation!();
}

impl Query for QueryBuilder<Relate> {
    fn construct(self) -> String {
        let (from, to) = if let Some((from, to)) = self.relation {
            (from, to)
        } else {
            panic!("A relation must be specified!")
        };

        let only = if self.only { " ONLY" } else { "" };

        let mut query = format!(
            "{}{} {}->{}->{}",
            Self::STATEMENT,
            only,
            from,
            self.table,
            to
        );

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
