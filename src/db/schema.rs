table! {
    keywords (uuid) {
        uuid -> Binary,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        label -> Varchar,
    }
}

table! {
    nafs (uuid) {
        uuid -> Binary,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        code -> Varchar,
        label -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    romes (uuid) {
        uuid -> Binary,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        code -> Varchar,
        label -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    rome_nafs (uuid) {
        uuid -> Binary,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        rome_uuid -> Binary,
        naf_uuid -> Binary,
    }
}

joinable!(rome_nafs -> nafs (naf_uuid));
joinable!(rome_nafs -> romes (rome_uuid));

allow_tables_to_appear_in_same_query!(
    keywords,
    nafs,
    romes,
    rome_nafs,
);
