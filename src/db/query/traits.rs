use crate::db::QueryBuilder;

pub trait Statement {
    const STATEMENT: &'static str;

    fn query(table: &str) -> QueryBuilder<Self>
    where
        Self: Sized,
    {
        QueryBuilder::new(table)
    }
}

pub trait Query {
    fn construct(self) -> String;
}

impl Query for String {
    fn construct(self) -> String {
        self
    }
}

impl Query for &str {
    fn construct(self) -> String {
        self.to_string()
    }
}
