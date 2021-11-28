#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::anatomy_questions;

pub mod handler;
pub mod repository;
pub mod router;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Clone)]
#[table_name = "anatomy_questions"]
pub struct Question {
    pub id: i32,
    pub label: String,
    pub kind: String,
    pub option_one: String,
    pub option_two: String,
    pub option_three: String,
    pub option_four: String,
    pub correct_answers: String,
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "anatomy_questions"]
pub struct InsertableQuestion {
    pub label: String,
    pub kind: String,
    pub option_one: String,
    pub option_two: String,
    pub option_three: String,
    pub option_four: String,
    pub correct_answers: String,
}