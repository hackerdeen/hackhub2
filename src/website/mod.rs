use actix_web::{get, web, Responder};
use diesel::prelude::*;

use crate::db::DbPool;
use crate::db::models::User;
use crate::db::schema::user::dsl::*;
use crate::user::get_user;

#[get("/")]
pub async fn index(pool: web::Data<DbPool>) -> impl Responder {
    let u = web::block(move || {
        let mut conn = pool.get().expect("Couldn't connect to database.");

        get_user(1, &mut *conn)
    });

    format!("{:?}", u.await.unwrap().unwrap())
}

#[get("/{name}")]
pub async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}
