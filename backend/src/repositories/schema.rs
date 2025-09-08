// @generated automatically by Diesel CLI.

diesel::table! {
    customer_service_categories (category) {
        #[max_length = 120]
        category -> Varchar,
    }
}

diesel::table! {
    customer_service_photos (id) {
        id -> Int8,
        #[max_length = 36]
        service_id -> Varchar,
        #[max_length = 2048]
        url -> Varchar,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        position -> Int4,
    }
}

diesel::table! {
    customer_service_tags (service_id, key) {
        #[max_length = 36]
        service_id -> Varchar,
        #[max_length = 120]
        key -> Varchar,
        #[max_length = 255]
        value -> Varchar,
    }
}

diesel::table! {
    customer_services (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        user_id -> Varchar,
        #[max_length = 120]
        name -> Varchar,
        #[max_length = 2048]
        description -> Varchar,
        latitude -> Float8,
        longitude -> Float8,
        #[max_length = 16]
        phone -> Varchar,
        #[max_length = 2048]
        website -> Nullable<Varchar>,
        deleted -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

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

diesel::joinable!(customer_service_photos -> customer_services (service_id));
diesel::joinable!(customer_service_tags -> customer_services (service_id));
diesel::joinable!(customer_services -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    customer_service_categories,
    customer_service_photos,
    customer_service_tags,
    customer_services,
    users,
);
