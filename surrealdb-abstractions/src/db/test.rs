#![cfg(test)]

use super::filter::*;
use super::query::*;
use crate::db::create::Create;
use crate::db::query::select::Select;
use crate::test::TEST_DB;
use crate::test::{db, init_db};
use log::info;
use std::sync::Arc;
use surrealdb::sql::Thing;
use tosic_logging_utils::init_test_logger;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
struct TestData {
    id: Option<Thing>,
    name: String,
    age: i32,
}

impl TestData {
    pub fn new(name: String, age: i32) -> Self {
        Self {
            id: None,
            name,
            age,
        }
    }

    pub fn create_query(&self) -> QueryBuilder<Create> {
        Create::query("test_data")
            .add_field_to_content("name", self.name.clone())
            .add_field_to_content("age", self.age)
    }

    pub fn select_all_query(&self) -> QueryBuilder<Select> {
        Select::query("test_data")
    }
}

impl Default for TestData {
    fn default() -> Self {
        Self {
            id: None,
            name: "John Doe".to_string(),
            age: 18,
        }
    }
}

#[test]
fn test() {
    let filter = Filter::default()
        .add_condition("username".to_string(), None, "admin")
        .add_condition("age".to_string(), Some(">".to_string()), 18);

    let query = QueryBuilder::<Select>::new("user")
        .set_filter(filter)
        .set_limit(1)
        .add_field("*", None)
        .add_field("name", Some("username"))
        .construct();

    let excepted =
        "SELECT *, name AS username FROM user WHERE age > 18 AND username = 'admin' LIMIT 1"
            .to_string();

    assert_eq!(query, excepted);
}

#[tokio::test]
async fn test_run_query() -> anyhow::Result<()> {
    let data = TestData::default();

    let db = Arc::new(db().await?);

    let res: Vec<TestData> = data.create_query().run(&db, 0).await?;

    println!("Created data: {:?}", res);

    let res: Vec<TestData> = data.select_all_query().run(&db, 0).await?;

    println!("Selected data: {:?}", res);

    Ok(())
}

#[tokio::test]
async fn test_relate() -> anyhow::Result<()> {
    init_test_logger("debug");
    let data = TestData::default();
    let from = TestData::new("Emil".to_string(), 69);

    let db = Arc::new(db().await?);

    let to: Vec<TestData> = data.create_query().run(&db, 0).await?;
    let from: Vec<TestData> = from.create_query().run(&db, 0).await?;

    let query = Relate::query("relation")
        .relate_items(from[0].clone().id.unwrap(), to[0].clone().id.unwrap())
        .add_field_to_content("type", "friendship")
        .set_parallel(true)
        .construct();

    db.query(query).await?;

    let res: Vec<TestData> = data.select_all_query().run(&db, 0).await?;

    info!("Selected data: {:?}", res);

    Ok(())
}

#[tokio::test]
async fn test_construct_create_query() -> anyhow::Result<()> {
    let data = TestData::default();

    let query = data
        .create_query()
        .add_field_to_content("type", "friendship")
        .construct();

    println!("{}", query);
    assert_eq!(
        query,
        "CREATE test_data CONTENT { age: 18,  name: 'John Doe',  type: 'friendship'}"
    );

    Ok(())
}

#[tokio::test]
async fn test_construct_select_query() -> anyhow::Result<()> {
    let data = TestData::default();

    let query = data.select_all_query().construct();

    println!("{}", query);
    assert_eq!(query, "SELECT * FROM test_data ".to_string());

    Ok(())
}
