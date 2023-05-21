use diesel::SqliteConnection;
use diesel::prelude::*;

use crate::db::models::User;

pub fn get_user(user_id: i32, conn: &mut SqliteConnection) -> Option<User> {
    use crate::db::schema::user::dsl::*;

    user
        .filter(id.eq(user_id))
        .load::<User>(&mut *conn)
        .unwrap()
        .first()
        .cloned()
}

