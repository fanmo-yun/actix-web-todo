use actix_session::{
    storage::CookieSessionStore, SessionMiddleware
};
use actix_web::{App, HttpServer, web, cookie::Key};
use actix_files as fs;
use tera::Tera;

mod api;
mod db;

use db::db_api::DB;
use api::main_api::get_scope;
use api::web_api::api_scope;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    DB::database_check().await.unwrap();

    let db = sqlx::sqlite::SqlitePool::connect("USERDATA.db").await.unwrap();

    let key = Key::generate();
    
    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(Tera::new("static/pages/**/*").unwrap()))
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(api_scope())
            .service(get_scope())
    })
    .bind(("127.0.0.1",8000))?
    .run()
    .await
}
