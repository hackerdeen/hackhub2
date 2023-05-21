// @generated automatically by Diesel CLI.

diesel::table! {
    address (id) {
        id -> Nullable<Integer>,
        user -> Integer,
        line_1 -> Text,
        line_2 -> Nullable<Text>,
        line_3 -> Nullable<Text>,
        post_town -> Text,
        postcode -> Text,
        country -> Text,
    }
}

diesel::table! {
    membership (id) {
        id -> Nullable<Integer>,
        user -> Nullable<Integer>,
        registered -> Nullable<Timestamp>,
        ceased -> Nullable<Date>,
        cessation_registered -> Nullable<Timestamp>,
    }
}

diesel::table! {
    person_data (id) {
        id -> Nullable<Integer>,
        user -> Integer,
        legal_name -> Integer,
        service_address -> Integer,
        residential_address -> Nullable<Integer>,
        state -> Text,
        submitted_time -> Nullable<Timestamp>,
        registrar_action_time -> Nullable<Timestamp>,
        registrar -> Nullable<Integer>,
        registrar_note -> Nullable<Text>,
    }
}

diesel::table! {
    person_name (id) {
        id -> Nullable<Integer>,
        user -> Integer,
        title -> Text,
        forename -> Text,
        surname -> Text,
        honours -> Nullable<Text>,
    }
}

diesel::table! {
    user (id) {
        id -> Nullable<Integer>,
        username -> Text,
        preferred_name -> Text,
        email -> Text,
        irc -> Nullable<Text>,
        github -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    address,
    membership,
    person_data,
    person_name,
    user,
);
