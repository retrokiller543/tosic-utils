use std::collections::BTreeMap;
use surrealdb::sql::Value;

#[inline]
pub(crate) fn content(content: Option<BTreeMap<String, Value>>, query: &mut String) -> () {
    if let Some(content) = content {
        query.push_str(" CONTENT {");

        let mut contents = Vec::new();

        for (field, value) in content {
            contents.push(format!(" {}: {}", field, value));
        }

        query.push_str(&contents.join(", "));
        query.push('}');
    }
}
