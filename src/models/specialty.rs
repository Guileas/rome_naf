use chrono::NaiveDateTime;
use crate::db::schema::specialtys;

#[derive(AsChangeset, Queryable, Identifiable)]
#[primary_key(uuid)]
#[column_name(uuid)]
pub struct Specialty {
    pub uuid: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub label: String,
    pub description: Option<String>,
    pub naf_id: Vec<u8>,
}

#[derive(Insertable, Debug)]
#[table_name = "specialtys"]
pub struct NewSpecialty<'a> {
    pub uuid: &'a Vec<u8>,
    pub created_at: &'a NaiveDateTime,
    pub updated_at: Option<&'a NaiveDateTime>,
    pub label: &'a String,
    pub description: Option<&'a String>,
    pub naf_id: &'a Vec<u8>,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "specialtys"]
pub struct UpdateSpecialty{
    pub uuid: Vec<u8>,
    pub updated_at: Option<NaiveDateTime>,
    pub label: String,
    pub description: Option<String>,
    pub naf_id: Vec<u8>,
}

