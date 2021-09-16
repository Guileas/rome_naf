use chrono::NaiveDateTime;
use crate::db::schema::romes;

#[derive(AsChangeset, Queryable, Identifiable)]
#[primary_key(uuid)]
#[column_name(uuid)]
pub struct Rome {
    pub uuid: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub code: String,
    pub label: String,
    pub description: Option<String>,
}

#[derive(Insertable)]
#[table_name = "romes"]
pub struct NewRome<'a> {
    pub uuid: &'a Vec<u8>,
    pub created_at: &'a NaiveDateTime,
    pub updated_at: Option<&'a NaiveDateTime>,
    pub code: &'a String,
    pub label: &'a String,
    pub description: Option<&'a String>
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "romes"]
pub struct UpdateRome{
    pub uuid: Vec<u8>,
    pub updated_at: Option<NaiveDateTime>,
    pub code: String,
    pub label: String,
    pub description: Option<String>,
}
