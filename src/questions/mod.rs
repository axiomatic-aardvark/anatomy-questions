#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::questions;

pub mod handler;
pub mod repository;
pub mod router;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Clone)]
#[table_name = "questions"]
pub struct Question {
    pub id: i32,
    pub label: String,
    pub option_one: String,
    pub option_two: String,
    pub option_three: String,
    pub option_four: String,
    pub correct_answer: String,
    pub kind: String
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "questions"]
pub struct InsertableQuestion {
    pub label: String,
    pub option_one: String,
    pub option_two: String,
    pub option_three: String,
    pub option_four: String,
    pub correct_answer: String,
    pub kind: String
}