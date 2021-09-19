table! {
    keywords (uuid) {
        uuid -> Binary,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        label -> Varchar,
    }
}

table! {
    keyword_nafs (uuid) {
        uuid -> Binary,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        keyword_uuid -> Binary,
        naf_uuid -> Binary,
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

joinable!(keyword_nafs -> keywords (keyword_uuid));
joinable!(keyword_nafs -> nafs (naf_uuid));
joinable!(rome_nafs -> nafs (naf_uuid));
joinable!(rome_nafs -> romes (rome_uuid));

allow_tables_to_appear_in_same_query!(
    keywords,
    keyword_nafs,
    nafs,
    romes,
    rome_nafs,
);
