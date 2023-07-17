use crate::establish_connection;
use crate::models::{LoginUser, NewUser, User};
use crate::schema::users::dsl::*;
use actix_session::Session;
use actix_web::error::ErrorUnauthorized;
use actix_web::web;
use actix_web::{get, post, HttpResponse, Responder};
use bcrypt::DEFAULT_COST;
use diesel::prelude::*;

#[derive(serde::Serialize)]
pub struct SessionDetails {
    pub user_id: u32,
}

pub fn check_auth(session: &Session) -> Result<u32, actix_web::Error> {
    match session.get::<u32>("user_id").unwrap() {
        Some(user_id) => Ok(user_id),
        None => Err(ErrorUnauthorized("User not logged in.")),
    }
}

#[post("/login")]
pub async fn login(session: Session, sent_user: web::Json<LoginUser>) -> impl Responder {
    let connection = &mut establish_connection();
    let user = users
        .filter(email.eq(sent_user.email.clone()))
        .first::<User>(connection)
        .expect("Error loading users");
    if bcrypt::verify(sent_user.password.clone(), &user.password).unwrap() {
        let _ = session.insert("user_id", user.id);
        session.renew();
        HttpResponse::Ok().body("Login successful")
    } else {
        HttpResponse::Ok().body("Login failed")
    }
}

#[post("/users")]
pub async fn create_user(sent_user: web::Json<NewUser>) -> impl Responder {
    let connection = &mut establish_connection();
    let new_user = NewUser {
        name: sent_user.name.clone(),
        email: sent_user.email.clone(),
        password: bcrypt::hash(sent_user.password.clone(), DEFAULT_COST).unwrap(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .returning(User::as_returning())
        .execute(connection)
        .expect("Error saving new user");

    HttpResponse::Ok().body("User created")
}

#[get("/users")]
pub async fn get_users() -> impl Responder {
    let connection = &mut establish_connection();
    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    let mut resp = String::from("Users:\n");
    for user in results {
        resp.push_str(&format!("{} - {}\n", user.id, user.name));
    }

    HttpResponse::Ok().body(resp)
}
