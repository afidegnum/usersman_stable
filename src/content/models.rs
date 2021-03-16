use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
extern crate chrono;
use chrono::prelude::*;
use chrono::{DateTime, Duration, Utc};

//To be added based on special query

#[derive(Serialize, Clone, Deserialize, PostgresMapper)]
#[pg_mapper(table = "content")]
pub struct Content {
    pub id: i32,
    pub type_id: i32,
    pub title: String,
    pub summary: String,
    pub details: String,
    pub submitted_date: chrono::DateTime<Utc>,
    pub modified_date: chrono::DateTime<Utc>,
}

#[derive(Serialize, Clone, Deserialize, PostgresMapper, Debug)]
#[pg_mapper(table = "content")]
pub struct CreateContent {
    pub type_id: i32,
    pub title: String,
    pub summary: String,
    pub details: String,
    pub submitted_date: chrono::DateTime<Utc>,
    pub modified_date: chrono::DateTime<Utc>,
}

#[derive(Serialize, Clone, Deserialize, PostgresMapper, Debug)]
#[pg_mapper(table = "content")]
pub struct CreateSampleContent {
    pub type_id: i32,
    pub title: String,
    pub summary: String,
    pub details: String,
}
