use super::*;
use crate::db::query::traits::Query;
use crate::{
    impl_fetch_fields, impl_fields, impl_filter, impl_group_all, impl_group_by, impl_limit,
    impl_omit_fields, impl_only, impl_order_by, impl_parallel, impl_start,
};
use log::debug;

#[derive(Clone)]
pub struct Select;

impl Statement for Select {
    const STATEMENT: &'static str = "SELECT";
}

impl QueryBuilder<Select> {
    impl_fields!();
    impl_limit!();
    impl_start!();
    impl_filter!();
    impl_order_by!();
    impl_group_by!();
    impl_group_all!();
    impl_omit_fields!();
    impl_only!();
    impl_fetch_fields!();
    impl_parallel!();
}

impl Query for QueryBuilder<Select> {
    fn construct(self) -> String {
        let mut query = String::new();

        // Start with the query type
        query.push_str(Self::STATEMENT);
        query.push(' ');

        // Fields to select
        let fields = if let Some(fields) = self.fields {
            fields
                .into_iter()
                .map(|(field, alias)| {
                    if let Some(alias) = alias {
                        format!("{} AS {}", field, alias)
                    } else {
                        field
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            "*".to_string()
        };

        query.push_str(&fields);

        if let Some(omit_fields) = self.omit_fields {
            query.push_str(format!(" OMIT {}", omit_fields.join(", ")).as_str());
        }

        // From which table
        query.push_str(" FROM");

        if self.only {
            query.push_str(" ONLY");
        }

        query.push_str(format!(" {} ", self.table).as_str());

        // Add WHERE clause if filter is not empty
        let filter_clause = self.filter.construct();
        if !filter_clause.is_empty() {
            query.push_str(&filter_clause);
        }

        // Group Clause
        if self.group_all {
            query.push_str(" GROUP ALL");
        } else if let Some(group_by) = self.group_by {
            query.push_str(format!(" GROUP BY {}", group_by.join(", ")).as_str());
        }

        // Order By clause
        if let Some(order) = self.order {
            query.push_str(format!(" ORDER BY {}", order.join(", ")).as_str());
        }

        // Limit clause
        if let Some(limit) = self.limit {
            query.push_str(format!(" LIMIT {}", limit).as_str());
        }

        // Start (Offset) clause
        if let Some(start) = self.start {
            query.push_str(format!(" START {}", start).as_str());
        }

        debug!("Constructed SQL: {}", query);

        query
    }
}
