// @generated automatically by Diesel CLI.

diesel::table! {
    admins (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        name -> Text,
    }
}

diesel::table! {
    cleaners (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        name -> Text,
    }
}

diesel::table! {
    rooms (id) {
        id -> Integer,
        cleaner -> Nullable<Integer>,
    }
}

diesel::joinable!(rooms -> cleaners (cleaner));

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    cleaners,
    rooms,
);
