// @generated automatically by Diesel CLI.

diesel::table! {
    person (id) {
        id -> Int4,
        wallet -> Int4,
        created -> Nullable<Timestamp>,
    }
}

diesel::table! {
    wallet (id) {
        id -> Int4,
        its -> Nullable<Int4>,
    }
}

diesel::joinable!(person -> wallet (wallet));

diesel::allow_tables_to_appear_in_same_query!(
    person,
    wallet,
);
