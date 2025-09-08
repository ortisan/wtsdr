// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        deleted -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}
