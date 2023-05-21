
use std::env;

use actix_web::{App, HttpServer, web};
use diesel::SqliteConnection;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");

    HttpServer::new(move || App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(api::user::index)
        .service(api::irc::nicks))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
