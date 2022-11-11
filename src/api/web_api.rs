use actix_web::{web, post, delete, Responder, Scope, HttpResponse};
use sqlx::sqlite::SqlitePool;
use actix_session::Session;
use crate::db::modles::NewToDo;

pub fn api_scope() -> Scope {
    web::scope("/api")
        .service(crate_todo)
        .service(delete_todo)
}

#[post("/app")]
async fn crate_todo(todo: web::Form<NewToDo>, pool: web::Data<SqlitePool>, session: Session) -> impl Responder {
    if session.get::<String>("username").unwrap().is_none() {
        return HttpResponse::BadRequest().finish();
    }
    let db = pool.as_ref();
    sqlx::query(&format!("insert into TODOLIST values ('{}', '{}', '{}')", session.get::<String>("username").unwrap().unwrap(), todo.todo_title, todo.date_time)).execute(db).await.unwrap();
    return HttpResponse::Found().append_header(("location", "/app")).finish();
}

#[delete("/app/{title}/{time}")]
async fn delete_todo(path: web::Path<(String, String)>, pool: web::Data<SqlitePool>, session: Session) -> impl Responder {
    if session.get::<String>("username").unwrap().is_none() {
        return HttpResponse::BadRequest().finish();
    }
    let db =pool.as_ref();
    let (title, time) = path.into_inner();
    let res = sqlx::query(&format!("delete from TODOLIST where username = '{}' and todotitle = '{}' and date_time = '{}'", session.get::<String>("username").unwrap().unwrap(), title, time)).execute(db).await.is_ok();
    if res { HttpResponse::Ok() } else { HttpResponse::ExpectationFailed() }.finish()
}