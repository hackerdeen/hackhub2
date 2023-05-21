use diesel::SqliteConnection;
use diesel::prelude::*;

use crate::db::models::{NewUser,User};

pub enum MembershipStatus {
    /// No membership records, including no active membership applications. This status will be
    /// returned if the only records are for rejected applications.
    NotMember,
    /// An application for membership has been submitted but has not yet been approved or rejected.
    ApplicationPending,
    /// Currently a member.
    CurrentMember,
    /// A previous membership record is found, but it has ceased.
    MembershipCeased,
}

pub fn new_user(new_user: &NewUser, conn: &mut SqliteConnection) -> User {
    use crate::db::schema::user;

    diesel::insert_into(user::table)
        .values(new_user)
        .get_result(conn)
        .expect("Error creating user")
}

pub fn get_user(flt_user_id: i32, conn: &mut SqliteConnection) -> Option<User> {
    use crate::db::schema::user::dsl::*;

    user
        .filter(id.eq(flt_user_id))
        .load::<User>(conn)
        .unwrap()
        .first()
        .cloned()
}

/// Returns a Vec of all IRC nicks for all users.
///
/// TODO: The previous implementation of this endpoint only returned *active members*.
pub fn get_irc_nicks(conn: &mut SqliteConnection) -> Vec<String> {
    use crate::db::schema::user::dsl::*;

    user
        .load::<User>(conn)
        .unwrap()
        .into_iter()
        .map(|u| u.irc)
        .filter(|i| i.is_some())
        .map(|i| i.unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use std::fs::remove_file;
    use std::io::ErrorKind;
    use diesel::{Connection, SqliteConnection};
    use diesel_migrations::MigrationHarness;
    use crate::db::models::{NewUser, User};
    use crate::MIGRATIONS;
    use crate::user::{get_user, new_user};

    fn new_database() -> SqliteConnection {
        use std::fs::remove_file;
        use rusqlite::Connection;
        if let Err(err) = remove_file("test.sqlite3") {
            if err.kind() != ErrorKind::NotFound {
                panic!("Could not remove existing test database.")
            }
        };
        Connection::open("test.sqlite3").expect("Could not create new test database.").close().expect("Could not close temporary database connection.");
        let mut conn = diesel::SqliteConnection::establish("test.db").expect("Could not connect to new database.");
        conn.run_pending_migrations(MIGRATIONS).expect("Could not run migrations.");
        conn
    }

    #[test]
    fn test_simple_add_user() {
        let mut conn = new_database();
        let created = new_user(&NewUser {
            username: "test",
            preferred_name: "Mr Test Testington",
            email: "test@example.com",
            irc: Some("test123"),
            github: None,
        }, &mut conn);
        assert!(created.id.is_some());
        let looked_up = get_user(created.id.unwrap(), &mut conn).unwrap();
        assert_eq!(looked_up.username, "test");
        assert_eq!(looked_up.preferred_name, "Mr Test Testington");
    }
}