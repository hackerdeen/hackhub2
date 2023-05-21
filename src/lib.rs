#![doc = include_str!("../README.md")]

use diesel::sqlite::SqliteConnection;
use diesel::r2d2;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub(crate) mod db;

pub mod user;
