use diesel::prelude::*;
use actix_web::{get, HttpResponse, Responder};
use crate::establish_connection;
use crate::models::User;
use crate::schema::users::dsl::*;

#[get ("/users")]
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
