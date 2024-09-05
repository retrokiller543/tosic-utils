use crate::db::query::traits::Statement;

pub struct Insert;

impl Statement for Insert {
    const STATEMENT: &'static str = "INSERT";
}
