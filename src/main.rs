use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use stash::users::{create_user, get_users, login, check_auth, SessionDetails};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/whoami")]
async fn whoami(session: Session) -> impl Responder {
    let user_id = check_auth(&session);
    match user_id {
        Ok(user_id) => {
            let session_details = SessionDetails { user_id };
            HttpResponse::Ok().json(session_details)
        }
        Err(_) => HttpResponse::Unauthorized().body("User not logged in"),
    }
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = Key::generate();
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    HttpServer::new(move || {
        App::new()
        .wrap(
            SessionMiddleware::builder(
                CookieSessionStore::default(), 
                secret_key.clone()
            )
        .cookie_secure(false)
        .build(),
    )
    .wrap(Logger::default())
    .service(hello)
    .service(get_users)
    .service(create_user)
    .service(login)
    .service(whoami)
    .route("/hey", web::get().to(manual_hello))
})
.bind(("127.0.0.1", 8080))?
.run()
.await
}
