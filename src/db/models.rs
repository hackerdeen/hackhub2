use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::Queryable;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum RegisterUpdateStatus {
    MemberSubmitted,
    RecordedByRegistrar,
    RejectedByRegistrar,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Address {
    id: Option<i32>,
    user: i32,
    line_1: String,
    line_2: Option<String>,
    line_3: Option<String>,
    post_town: String,
    postcode: String,
    country: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Membership {
    id: Option<i32>,
    user: Option<i32>,
    registered: NaiveDateTime,
    ceased: NaiveDate,
    cessation_registered: NaiveDateTime,
}

#[derive(Debug, Serialize, Queryable)]
pub struct PersonData {
    id: Option<i32>,
    user: Option<i32>,
    legal_name: i32,
    service_address: i32,
    residential_address: Option<i32>,
    state: RegisterUpdateStatus,
    submitted_time: NaiveDateTime,
    registrar_action_time: NaiveDateTime,
    registrar: Option<i32>,
    registrar_note: Option<String>,
}

#[derive(Debug, Queryable)]
/// Legal name structured according to the Companies House XML schema. Names belong to users and
/// may be updated for name changes. Where a name change occurs due to the issue of a Gender
/// Recognition Certificate, it is an offence to disclose former names (Gender Recognition Act 2004
/// s. 22). The only exemption to this is if the member is an office holder, as Companies House
/// has a duty to maintain a public register of officers per Companies Act 2006.
pub struct PersonName {
    pub id: Option<i32>,
    /// *Foreign key*. Link to user table.
    pub user: Option<i32>,
    ///
    pub title: String,
    pub forename: String,
    pub surname: String,
    pub honours: Option<String>,
}


#[derive(Clone, Debug, Serialize, Queryable)]
pub struct User {
    pub id: Option<i32>,
    /// Used to identify the user account.
    pub username: String,
    /// Used for all purposes other than those relating to authentication, and those requiring the
    /// use of a "legal name".
    pub preferred_name: String,
    /// Used for legal notices, and other communications if opted-in.
    pub email: String,
    pub irc: Option<String>,
    pub github: Option<String>,
}
