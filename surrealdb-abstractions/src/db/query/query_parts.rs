use crate::db::query::ContentType;
use crate::prelude::Filter;
use std::collections::BTreeMap;
use surrealdb::sql::Value;

#[inline]
pub(crate) fn content(
    content_type: ContentType,
    content: Option<BTreeMap<String, Value>>,
    query: &mut String,
) {
    if let Some(content) = content {
        match content_type {
            ContentType::Content => query.push_str("CONTENT {"),
            ContentType::Merge => query.push_str("Merge {"),
            ContentType::Patch => query.push_str("PATCH {"),
        }

        let mut contents = Vec::new();

        for (field, value) in content {
            contents.push(format!(" {}: {}", field, value));
        }

        query.push_str(&contents.join(", "));
        query.push('}');
    }
}

#[inline]
pub(crate) fn filter(filter: Filter, query: &mut String) {
    let filter_clause = filter.construct();
    if !filter_clause.is_empty() {
        query.push_str(&filter_clause);
    }
}

#[inline]
pub(crate) fn timeout(timeout: Option<(usize, String)>, query: &mut String) {
    if let Some((duration, unit)) = timeout {
        query.push_str(format!(" TIMEOUT {}{}", duration, unit).as_str());
    }
}

#[inline]
pub(crate) fn parallel(parallel: bool, query: &mut String) {
    if parallel {
        query.push_str(" PARALLEL");
    }
}
