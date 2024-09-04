use std::collections::BTreeMap;
use surrealdb::sql::Value;

type FilterField = String;
type FilterOperator = Option<String>;
type FilterKey = (FilterField, FilterOperator);
type FilterValue = Value;
type InnerFilter = BTreeMap<FilterKey, FilterValue>;

/// Returns the WHERE clause for the statement based on the filter. If the filter is empty, it returns an empty string.
///
/// # Arguments
///
/// * `filter` - The filter to apply to the statement.
///
/// ## Filter Layout
///
/// The filter is a map of key-value pairs, where the key is a tuple of (key, operator) and the value is the value to filter by.
///
/// # Limits
///
/// The filter will join the conditions with an AND. Currently, there is no support for OR.
#[deprecated(since = "0.1.0", note = "use Filter struct instead")]
pub fn get_filter_query(filter: &InnerFilter) -> String {
    let mut condition = String::from("WHERE ");

    let mut conditions = Vec::new();

    for ((key, operator), value) in filter {
        conditions.push(format!(
            "{} {} {}",
            key,
            operator.clone().unwrap_or("=".to_string()),
            value
        ));
    }

    if !conditions.is_empty() {
        condition.push_str(&conditions.join(" AND "));
    } else {
        condition = String::new();
    }

    condition
}

/// Represents a filter for the database.
///
/// # Examples
///
/// ```
/// # use surrealdb_abstraction::db::Filter;
///
/// let filter = Filter::new()
///     .add_condition("username".to_string(), None, "admin")
///     .add_condition("age".to_string(), Some(">".to_string()), 18)
///     .construct();
///
/// assert_eq!(filter, "WHERE age > 18 AND username = 'admin'".to_string());
/// ```
///
#[derive(Clone, Debug)]
pub struct Filter(InnerFilter);

impl Filter {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn add_condition<T>(
        mut self,
        field: FilterField,
        operator: FilterOperator,
        value: T,
    ) -> Self
    where
        Value: From<T>,
    {
        let value = Value::from(value);

        self.0.insert((field, operator), value);

        self
    }

    pub fn construct(self) -> String {
        let mut condition = String::from("WHERE ");

        let mut conditions = Vec::new();

        for ((key, operator), value) in self.0 {
            conditions.push(format!(
                "{} {} {}",
                key,
                operator.unwrap_or("=".to_string()),
                value
            ));
        }

        if !conditions.is_empty() {
            condition.push_str(&conditions.join(" AND "));
        } else {
            condition = String::new();
        }

        condition
    }
}

impl From<Filter> for InnerFilter {
    fn from(filter: Filter) -> Self {
        filter.0
    }
}

impl From<InnerFilter> for Filter {
    fn from(filter: InnerFilter) -> Self {
        Self(filter)
    }
}
