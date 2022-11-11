use crate::db::modles::{User, EnrollUser, ToDo};
use actix_web::{web, post, HttpResponse, get, Responder, Scope};
use actix_session::Session;
use actix_files::NamedFile;
use tera::{Tera, Context};
use sqlx::{SqlitePool, Row};

pub fn get_scope() -> Scope {
    web::scope("")
        .service(index)
        .service(login)
        .service(enroll)
        .service(about)
        .service(process_login)
        .service(process_enroll)
        .service(process_logout)
        .service(apps)
}

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("./static/pages/index.html").await.unwrap()
}

#[get("/login")]
async fn login(session: Session, tera: web::Data<Tera>) -> impl Responder {
    if let Ok(Some(state)) = session.get::<String>("user_state") {
        if state.as_str() == "successful" {
            HttpResponse::Found().append_header(("location", "/app")).finish()
        } else {
            let mut context = Context::new();
            context.insert("todo", &"simple todo");
            context.insert("status", &"login failed");
            HttpResponse::Ok().body(tera.render("login.html", &context).unwrap())
        }
    } else {
        let mut context = Context::new();
        context.insert("todo", &"simple todo");
        context.insert("status", &"");
        HttpResponse::Ok().body(tera.render("login.html", &context).unwrap())
    }
}

#[post("/login")]
async fn process_login(pool: web::Data<SqlitePool> ,user: web::Form<User>, session: Session) -> impl Responder {
    let db = pool.as_ref();
    match sqlx::query(&format!("select * from USERDATAS where username is '{}'",user.username)).fetch_one(db).await {
        Ok(result) => {
            let tempname: String = result.get(0);
            let temppasswd: String = result.get(1);
            if user.username == tempname && user.passwd == temppasswd {
                session.insert("username", tempname).unwrap();
                session.insert("user_state", "successful".to_string()).unwrap();
                return HttpResponse::Found().append_header(("location", "/app")).finish();
            } else {
                session.insert("user_state", "incorrect password".to_string()).unwrap();
                return HttpResponse::Found().append_header(("location", "/login")).finish();
            };
        },
        Err(_) => {
            session.insert("user_state", "unknown user".to_string()).unwrap();
            return HttpResponse::Found().append_header(("location", "/login")).finish();
        },
    };
}

#[get("/enroll")]
async fn enroll() -> impl Responder {
    NamedFile::open_async("./static/pages/enroll.html").await.unwrap()
}

#[post("/enroll")]
async fn process_enroll(pool: web::Data<SqlitePool> ,user: web::Form<EnrollUser>) -> impl Responder {
    let db = pool.as_ref();
    sqlx::query(&format!("insert into USERDATAS values ('{}', '{}')", user.eusername, user.epasswd)).execute(db).await.unwrap();
    HttpResponse::Found().append_header(("location", "/")).finish()
}

#[get("/about")]
async fn about() -> impl Responder {
    NamedFile::open_async("./static/pages/about.html").await.unwrap()
}

#[get("/app")]
async fn apps(session: Session, tera: web::Data<Tera>, pool: web::Data<SqlitePool>) -> impl Responder {
    let db = pool.as_ref();
    if session.get::<String>("user_state").unwrap().is_none() {
        return HttpResponse::Found().append_header(("location", "/login")).finish();
    }
    
    let mut context = Context::new();
    context.insert("username", &session.get::<String>("username").unwrap().unwrap());
    
    match sqlx::query(&format!("select * from TODOLIST where username = '{}'", session.get::<String>("username").unwrap().unwrap())).fetch_all(db).await {
        Ok(todos) => {
            let mut v = Vec::new();
            for todo in todos {
                v.push(
                    ToDo {
                        todo_title: todo.get(1),
                        date_time: todo.get(2)
                    }
                );
            }
            context.insert("have_todo", &true);
            context.insert("todes", &v);
        },
        Err(_) => {
            context.insert("have_todo", &false);
        }
    };

    return HttpResponse::Ok().body(tera.render("apps.html", &context).unwrap());
}

#[get("/logout")]
async fn process_logout(session: Session) -> impl Responder {
    if session.get::<String>("user_state").unwrap().is_none() || session.get::<String>("username").unwrap().is_none() {
        return HttpResponse::NotFound().finish();
    }
    session.purge();
    return HttpResponse::Found().append_header(("location", "/")).finish();
}
