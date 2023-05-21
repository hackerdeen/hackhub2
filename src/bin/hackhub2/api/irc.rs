use actix_web::{get, Responder, web};

use hackhub2::DbPool;
use hackhub2::user::get_irc_nicks;

#[get("/api/irc/nicks")]
pub async fn nicks(pool: web::Data<DbPool>) -> impl Responder {
    let u = web::block(move || {
        let mut conn = pool.get().expect("Couldn't connect to database.");

        get_irc_nicks(&mut *conn)
    });

    format!("{:?}", u.await.unwrap())
}
