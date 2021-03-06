use crate::anatomy_questions;
use crate::anatomy_questions::Question;
use crate::anatomy_questions::InsertableQuestion;
use crate::connection::DbConn;
use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use std::env;

#[get("/anatomy")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Question>>, Status> {
    anatomy_questions::repository::all(&connection)
        .map(|questions| Json(questions))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    println!("{:?}", error);
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/anatomy/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Question>, Status> {
    anatomy_questions::repository::get(id, &connection)
        .map(|question| Json(question))
        .map_err(|error| error_status(error))
}

#[get("/anatomy/name/<label>")]
pub fn find_by_name(label: String, connection: DbConn) -> Result<Json<Vec<Question>>, Status> {
    anatomy_questions::repository::find_by_label(label, &connection)
        .map(|question| Json(question))
        .map_err(|error| error_status(error))
}

#[get("/anatomy/kind/<kind>")]
pub fn find_by_kind(kind: String, connection: DbConn) -> Result<Json<Vec<Question>>, Status> {
    anatomy_questions::repository::find_by_kind(kind, &connection)
        .map(|question| Json(question))
        .map_err(|error| error_status(error))
}

#[get("/anatomy/random")]
pub fn rand(connection: DbConn) -> Result<Json<Question>, Status> {
    anatomy_questions::repository::rand(&connection)
        .map(|question| Json(question))
        .map_err(|error| error_status(error))
}

#[post("/anatomy", format = "application/json", data = "<question>")]
pub fn post(
    question: Json<InsertableQuestion>,
    connection: DbConn,
) -> Result<status::Created<Json<Question>>, Status> {
    anatomy_questions::repository::insert(question.into_inner(), &connection)
        .map(|question| question_created(question))
        .map_err(|error| error_status(error))
}

fn question_created(question: Question) -> status::Created<Json<Question>> {
    status::Created(
        format!(
            "{host}:{port}/anatomy_questions/{id}",
            host = host(),
            port = port(),
            id = question.id
        )
        .to_string(),
        Some(Json(question)),
    )
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/anatomy/<id>", format = "application/json", data = "<question>")]
pub fn put(
    id: i32,
    question: Json<InsertableQuestion>,
    connection: DbConn,
) -> Result<Json<Question>, Status> {
    anatomy_questions::repository::update(id, question.into_inner(), &connection)
        .map(|question| Json(question))
        .map_err(|error| error_status(error))
}

#[delete("/anatomy/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match anatomy_questions::repository::get(id, &connection) {
        Ok(_) => anatomy_questions::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error)),
    }
}
