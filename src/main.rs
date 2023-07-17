use std::sync::Mutex;

use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{
    get,
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

use aws_sdk_s3::Client;

use stash::settings::Settings;
use stash::users::{check_auth, create_user, get_users, login, SessionDetails};

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

#[get("/list_buckets")]
async fn list_buckets(s3_client: Data<Mutex<Client>>) -> impl Responder {
    let s3_client = s3_client.lock().unwrap();
    let resp = s3_client.list_buckets().send().await.unwrap();
    HttpResponse::Ok().body(format!("{:#?}", resp))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn find_or_create_bucket(s3_client: &Client, bucket_name: &str) -> String {
    let resp = s3_client
        .list_buckets()
        .send()
        .await
        .expect("Couldn't list buckets");
    let bucket_names: Vec<String> = resp
        .buckets
        .unwrap_or_default()
        .into_iter()
        .map(|bucket| bucket.name.unwrap_or_default())
        .collect();
    if bucket_names.contains(&bucket_name.to_string()) {
        bucket_name.to_string()
    } else {
        let resp = s3_client
            .create_bucket()
            .bucket(bucket_name)
            .send()
            .await
            .expect("Couldn't create bucket");
        resp.location.unwrap_or_default()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let stash_config = Settings::new().expect("Couldn't load config");
    let aws_app_config = aws_config::from_env().region("us-east-1").load().await;
    let s3_client = Client::new(&aws_app_config);
    let _bucket_name = find_or_create_bucket(&s3_client, &stash_config.s3_storage.bucket).await;
    let s3_app_client = Data::new(Mutex::new(s3_client));
    let _settings = Settings::new().expect("Couldn't load config");
    let secret_key = Key::generate();

    // let bucket_name = env!()

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(s3_app_client.clone())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .wrap(Logger::default())
            .service(hello)
            .service(get_users)
            .service(create_user)
            .service(login)
            .service(whoami)
            .service(list_buckets)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
