use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use stash::users::{create_user, get_users, login};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/echo/{name}")]
async fn echo(name: web::Path<String>) -> impl Responder {
    let resp = format!("Echo: {}", name);
    HttpResponse::Ok().body(resp)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .service(get_users)
            .service(create_user)
            .service(login)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
