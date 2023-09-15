// @generated automatically by Diesel CLI.

diesel::table! {
    keyword_nafs (uuid) {
        uuid -> Binary,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        keyword_uuid -> Binary,
        naf_uuid -> Binary,
    }
}

diesel::table! {
    keywords (uuid) {
        uuid -> Binary,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        label -> Varchar,
    }
}

diesel::table! {
    nafs (uuid) {
        uuid -> Binary,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        code -> Varchar,
        label -> Varchar,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    rome_nafs (uuid) {
        uuid -> Binary,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        rome_uuid -> Binary,
        naf_uuid -> Binary,
    }
}

diesel::table! {
    romes (uuid) {
        uuid -> Binary,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        code -> Varchar,
        label -> Varchar,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    specialtys (uuid) {
        uuid -> Binary,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        label -> Varchar,
        description -> Nullable<Varchar>,
        naf_id -> Binary,
    }
}

diesel::joinable!(keyword_nafs -> keywords (keyword_uuid));
diesel::joinable!(keyword_nafs -> nafs (naf_uuid));
diesel::joinable!(rome_nafs -> nafs (naf_uuid));
diesel::joinable!(rome_nafs -> romes (rome_uuid));
diesel::joinable!(specialtys -> nafs (naf_id));

diesel::allow_tables_to_appear_in_same_query!(
    keyword_nafs,
    keywords,
    nafs,
    rome_nafs,
    romes,
    specialtys,
);
