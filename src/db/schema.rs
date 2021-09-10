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

allow_tables_to_appear_in_same_query!(
    nafs,
);
