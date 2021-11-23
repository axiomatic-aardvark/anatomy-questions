use crate::questions;
use crate::questions::Question;
use crate::questions::InsertableQuestion;
use crate::connection::DbConn;
use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use std::env;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Question>>, Status> {
    questions::repository::all(&connection)
        .map(|questions| Json(questions))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Question>, Status> {
    questions::repository::get(id, &connection)
        .map(|question| Json(question))
        .map_err(|error| error_status(error))
}

#[get("/name/<label>")]
pub fn find_by_name(label: String, connection: DbConn) -> Result<Json<Vec<Question>>, Status> {
    questions::repository::find_by_label(label, &connection)
        .map(|question| Json(question))
        .map_err(|error| error_status(error))
}

#[get("/kind/<kind>")]
pub fn find_by_kind(kind: String, connection: DbConn) -> Result<Json<Vec<Question>>, Status> {
    questions::repository::find_by_kind(kind, &connection)
        .map(|question| Json(question))
        .map_err(|error| error_status(error))
}

#[get("/anatomy")]
pub fn get_anatomy(connection: DbConn) -> Result<Json<Vec<Question>>, Status> {
    questions::repository::get_anatomy(&connection)
        .map(|question| Json(question))
        .map_err(|error| error_status(error))
}

#[get("/random")]
pub fn rand(connection: DbConn) -> Result<Json<Question>, Status> {
    questions::repository::rand(&connection)
        .map(|question| Json(question))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<question>")]
pub fn post(
    question: Json<InsertableQuestion>,
    connection: DbConn,
) -> Result<status::Created<Json<Question>>, Status> {
    questions::repository::insert(question.into_inner(), &connection)
        .map(|question| question_created(question))
        .map_err(|error| error_status(error))
}

fn question_created(question: Question) -> status::Created<Json<Question>> {
    status::Created(
        format!(
            "{host}:{port}/questions/{id}",
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

#[put("/<id>", format = "application/json", data = "<question>")]
pub fn put(
    id: i32,
    question: Json<InsertableQuestion>,
    connection: DbConn,
) -> Result<Json<Question>, Status> {
    questions::repository::update(id, question.into_inner(), &connection)
        .map(|question| Json(question))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match questions::repository::get(id, &connection) {
        Ok(_) => questions::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error)),
    }
}
