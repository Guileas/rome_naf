use chrono::NaiveDateTime;
use crate::db::schema::keywords;

#[derive(AsChangeset, Queryable, Identifiable)]
#[primary_key(uuid)]
#[column_name(uuid)]
pub struct Keyword {
    pub uuid: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub label: String,
}

#[derive(Insertable)]
#[table_name = "keywords"]
pub struct NewKeyword<'a> {
    pub uuid: &'a Vec<u8>,
    pub created_at: &'a NaiveDateTime,
    pub updated_at: Option<&'a NaiveDateTime>,
    pub label: &'a String,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "keywords"]
pub struct UpdateKeyword{
    pub uuid: Vec<u8>,
    pub updated_at: Option<NaiveDateTime>,
    pub label: String,
}
