pub mod create;
pub mod delete;
pub mod insert;
pub mod select;
pub mod update;
pub mod traits;
pub mod relate;
mod query_parts;

pub use create::*;
pub use delete::*;
pub use insert::*;
pub use select::*;
pub use update::*;
pub use relate::*;
pub use traits::*;

use super::filter::Filter;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use surrealdb::sql::{Thing, Value};

#[derive(Clone, Debug)]
pub struct QueryBuilder<Type>
where
    Type: Statement + ?Sized,
{
    /// Fields to return (field, alias)
    fields: Option<BTreeMap<String, Option<String>>>,
    /// Fields to omit
    omit_fields: Option<Vec<String>>,
    /// Expect only 1 result
    only: bool,
    /// Table name
    table: String,
    relation: Option<(Thing, Thing)>,

    /// Filters (WHERE)
    filter: Filter,
    content: Option<BTreeMap<String, Value>>,

    /// Orders (ORDER BY)
    order: Option<Vec<String>>,
    /// Groups (GROUP BY)
    group_by: Option<Vec<String>>,
    group_all: bool,

    /// Limit
    limit: Option<u64>,
    /// Start (OFFSET)
    start: Option<u64>,

    /// Fields to fetch
    fetch_fields: Option<Vec<String>>,

    timeout: Option<(usize, String)>,

    /// Parallel query
    parallel: bool,

    phantom_data: PhantomData<Type>,
}

impl<Type> QueryBuilder<Type>
where
    Type: Statement,
{
    const STATEMENT: &'static str = Type::STATEMENT;

    pub fn new(table: &str) -> Self {
        Self {
            fields: None,
            table: table.to_string(),
            filter: Filter::new(),
            content: None,
            order: None,
            group_by: None,
            group_all: false,
            limit: None,
            start: None,
            fetch_fields: None,
            only: false,
            omit_fields: None,
            timeout: None,
            parallel: false,
            relation: None,

            phantom_data: PhantomData,
        }
    }

    #[inline]
    fn add_field_internal(mut self, field: String, alias: Option<String>) -> Self {
        self.fields
            .get_or_insert_with(BTreeMap::new)
            .insert(field, alias);

        self
    }

    #[inline]
    fn add_fields_internal(mut self, fields: BTreeMap<String, Option<String>>) -> Self {
        self.fields.get_or_insert_with(BTreeMap::new).extend(fields);

        self
    }

    #[inline]
    fn set_only_internal(mut self, only: bool) -> Self {
        self.only = only;

        self
    }

    #[inline]
    fn set_fetch_fields_internal(mut self, fetch_fields: Vec<String>) -> Self {
        self.fetch_fields = Some(fetch_fields);

        self
    }

    #[inline]
    fn set_fetch_field_internal(mut self, fetch_fields: String) -> Self {
        self.fetch_fields
            .get_or_insert_with(Vec::new)
            .push(fetch_fields);

        self
    }

    #[inline]
    fn set_omit_fields_internal(mut self, omit_fields: Vec<String>) -> Self {
        self.omit_fields = Some(omit_fields);

        self
    }

    #[inline]
    fn set_omit_field_internal(mut self, omit_fields: String) -> Self {
        self.omit_fields
            .get_or_insert_with(Vec::new)
            .push(omit_fields);

        self
    }

    #[inline]
    fn set_parallel_internal(mut self, parallel: bool) -> Self {
        self.parallel = parallel;

        self
    }

    #[inline]
    fn set_limit_internal(mut self, limit: u64) -> Self {
        self.limit = Some(limit);

        self
    }

    #[inline]
    fn set_start_internal(mut self, start: u64) -> Self {
        self.start = Some(start);

        self
    }

    #[inline]
    fn set_filter_internal(mut self, filter: Filter) -> Self {
        self.filter = filter;

        self
    }

    #[inline]
    fn add_condition_internal<T>(
        mut self,
        field: String,
        operator: Option<String>,
        value: T,
    ) -> Self
    where
        Value: From<T>,
    {
        self.filter = self.filter.add_condition(field, operator, value);

        self
    }

    #[inline]
    fn order_by_internal(mut self, order: Vec<String>) -> Self {
        self.order = Some(order);

        self
    }

    #[inline]
    fn group_by_internal(mut self, group_by: Vec<String>) -> Self {
        self.group_by = Some(group_by);

        self
    }

    #[inline]
    fn group_all_internal(mut self) -> Self {
        self.group_all = true;

        self
    }

    #[inline]
    fn set_timeout_internal(mut self, timeout: (usize, String)) -> Self {
        self.timeout = Some(timeout);

        self
    }

    #[inline]
    fn add_field_to_content_internal(mut self, field: String, content: Value) -> Self {
        self.content
            .get_or_insert_with(BTreeMap::new)
            .insert(field, content);

        self
    }

    #[inline]
    fn set_content_internal(mut self, content: BTreeMap<String, Value>) -> Self {
        self.content = Some(content);

        self
    }

    #[inline]
    fn relate_items_internal(mut self, from: Thing, to: Thing ) -> Self {
        self.relation = Some((from, to));

        self
    }
}

#[macro_export]
macro_rules! impl_filter {
    () => {
        pub fn set_filter(self, filter: Filter) -> Self {
            self.set_filter_internal(filter)
        }

        pub fn add_condition<T>(self, field: &str, operator: Option<&str>, value: T) -> Self
        where
            Value: From<T>,
        {
            let operator = operator.map(|o| o.to_string());
            self.add_condition_internal(field.to_string(), operator, value)
        }
    };
}

#[macro_export]
macro_rules! impl_limit {
    () => {
        pub fn set_limit(self, limit: u64) -> Self {
            self.set_limit_internal(limit)
        }
    };
}

#[macro_export]
macro_rules! impl_start {
    () => {
        pub fn set_start(self, start: u64) -> Self {
            self.set_start_internal(start)
        }
    };
}

#[macro_export]
macro_rules! impl_order_by {
    () => {
        pub fn order_by(self, order: Vec<&str>) -> Self {
            self.order_by_internal(order.iter().map(|s| s.to_string()).collect())
        }
    };
}

#[macro_export]
macro_rules! impl_group_by {
    () => {
        pub fn group_by(self, group_by: Vec<&str>) -> Self {
            self.group_by_internal(group_by.iter().map(|s| s.to_string()).collect())
        }
    };
}

#[macro_export]
macro_rules! impl_fields {
    () => {
        pub fn add_field(self, field: &str, alias: Option<&str>) -> Self {
            self.add_field_internal(field.to_string(), alias.map(|s| s.to_string()))
        }

        pub fn add_fields(self, fields: BTreeMap<String, Option<String>>) -> Self {
            self.add_fields_internal(fields)
        }
    };
}

#[macro_export]
macro_rules! impl_only {
    () => {
        pub fn set_only(self, only: bool) -> Self {
            self.set_only_internal(only)
        }
    };
}

#[macro_export]
macro_rules! impl_fetch_fields {
    () => {
        pub fn set_fetch_fields(self, fetch_fields: Vec<&str>) -> Self {
            self.set_fetch_fields_internal(fetch_fields.iter().map(|s| s.to_string()).collect())
        }

        pub fn set_fetch_field(self, fetch_field: &str) -> Self {
            self.set_fetch_field_internal(fetch_field.to_string())
        }
    };
}

#[macro_export]
macro_rules! impl_omit_fields {
    () => {
        pub fn set_omit_fields(self, omit_fields: Vec<&str>) -> Self {
            self.set_omit_fields_internal(omit_fields.iter().map(|s| s.to_string()).collect())
        }

        pub fn set_omit_field(self, omit_field: &str) -> Self {
            self.set_omit_field_internal(omit_field.to_string())
        }
    };
}

#[macro_export]
macro_rules! impl_parallel {
    () => {
        pub fn set_parallel(self, parallel: bool) -> Self {
            self.set_parallel_internal(parallel)
        }
    };
}

#[macro_export]
macro_rules! impl_group_all {
    () => {
        pub fn group_all(self) -> Self {
            self.group_all_internal()
        }
    };
}

#[macro_export]
macro_rules! impl_timeout {
    () => {
        pub fn set_timeout(self, timeout: (usize, &str)) -> Self {
            let timeout = (timeout.0, timeout.1.to_string());
            self.set_timeout_internal(timeout)
        }
    };
}

#[macro_export]
macro_rules! impl_content {
    () => {
        pub fn add_field_to_content<T>(self, field: &str, content: T) -> Self where Value: From<T>, T: Into<Value> {
            self.add_field_to_content_internal(field.to_string(), content.into())
        }

        pub fn set_content(self, content: BTreeMap<String, Value>) -> Self {
            self.set_content_internal(content)
        }
    };
}

#[macro_export]
macro_rules! impl_relation {
    () => {
        pub fn relate_items(self, from: Thing, to: Thing) -> Self {
            self.relate_items_internal(from, to)
        }
    };
}
