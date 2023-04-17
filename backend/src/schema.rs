table! {
    login_history (id) {
        id -> Int4,
        user_id -> Int4,
        login_timestamp -> Timestamp,
    }
}

table! {
    people (id) {
        id -> Int4,
        name -> Varchar,
        gender -> Bool,
        age -> Int4,
        address -> Varchar,
        phone -> Varchar,
        email -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        login_session -> Varchar,
    }
}

table! {
    schematics (id) {
        id -> VarChar,
        title -> Varchar,
        description -> Varchar,
        author -> Int4,
        tags -> Varchar,
        display -> Varchar,
        date -> Timestamp,
    }
}

table! {
    comments (id) {
        id -> VarChar,
        schematic_id -> VarChar,
        rating -> Int4,
        body -> Text,
        author -> Int4,
        date -> Timestamp,
    }
}

joinable!(login_history -> users (user_id));
joinable!(comments -> schematics (schematic_id));

allow_tables_to_appear_in_same_query!(login_history, people, users, schematics, );
