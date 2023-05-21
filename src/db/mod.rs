
pub mod schema;
pub mod models;

use diesel::sqlite::SqliteConnection;
use diesel::r2d2;

pub(crate) type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;
