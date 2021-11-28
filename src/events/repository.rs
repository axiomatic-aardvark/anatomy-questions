#![allow(proc_macro_derive_resolution_fallback)]

use crate::anatomy_questions::Question;
use crate::anatomy_questions::InsertableQuestion;
use crate::schema::anatomy_questions;
use diesel;
use diesel::prelude::*;
use rand::prelude::*;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Question>> {
    anatomy_questions::table.load::<Question>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Question> {
    anatomy_questions::table.find(id).get_result::<Question>(connection)
}

pub fn find_by_label(label: String, connection: &PgConnection) -> QueryResult<Vec<Question>> {
    let all = anatomy_questions::table.load::<Question>(&*connection);
    match all {
        Ok(all) => {
            let matches = all
                .into_iter()
                .filter(|b| {
                    b.label.to_lowercase() == label.to_lowercase()
                        || b.label.to_lowercase().contains(label.to_lowercase().as_str())
                })
                .collect::<Vec<Question>>();
            return Ok(matches);
        }
        Err(e) => Err(e),
    }
}

pub fn find_by_kind(kind: String, connection: &PgConnection) -> QueryResult<Vec<Question>> {
    let all = anatomy_questions::table.load::<Question>(&*connection);
    match all {
        Ok(all) => {
            let matches = all
                .into_iter()
                .filter(|b| {
                    b.kind.to_lowercase() == kind.to_lowercase()
                        || b.kind.to_lowercase().contains(kind.to_lowercase().as_str())
                })
                .collect::<Vec<Question>>();
            return Ok(matches);
        }
        Err(e) => Err(e),
    }
}

pub fn rand(connection: &PgConnection) -> QueryResult<Question> {
    let mut rng = rand::thread_rng();
    let all = anatomy_questions::table.load::<Question>(&*connection);

    match all {
        Ok(all) => {
            let len = all.len();
            let x: usize = rng.gen_range(0..len);
            return Ok(all[x].clone());
        }
        Err(e) => Err(e),
    }
}

pub fn insert(question: InsertableQuestion, connection: &PgConnection) -> QueryResult<Question> {
    diesel::insert_into(anatomy_questions::table)
        .values(question)
        .get_result(connection)
}

pub fn update(id: i32, question: InsertableQuestion, connection: &PgConnection) -> QueryResult<Question> {
    diesel::update(anatomy_questions::table.find(id))
        .set(&question)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(anatomy_questions::table.find(id)).execute(connection)
}
