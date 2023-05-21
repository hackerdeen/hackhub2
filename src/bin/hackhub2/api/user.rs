use actix_web::{get, Responder, web};
use hackhub2::DbPool;
use hackhub2::user::get_user;

#[get("/api/user/{user_id}")]
pub async fn index(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> impl Responder {
    let loc_user = web::block(move || {
        let mut conn = pool.get().expect("Couldn't connect to database.");
        get_user(user_id.into_inner(), &mut *conn)
    });
    format!("{:?}", loc_user.await.unwrap().unwrap())
}
