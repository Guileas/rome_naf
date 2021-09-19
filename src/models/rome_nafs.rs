use chrono::NaiveDateTime;
use crate::db::schema::rome_nafs;
use crate::models::rome::Rome;
use crate::models::naf::Naf;

#[derive(AsChangeset, Queryable, Identifiable)]
#[primary_key(uuid)]
#[column_name(uuid)]
pub struct RomeNaf {
    pub uuid: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub rome_uuid: Vec<u8>,
    pub naf_uuid: Vec<u8>
}

#[derive(Insertable)]
#[table_name = "rome_nafs"]
pub struct NewRomeNaf<'a> {
    pub uuid: &'a Vec<u8>,
    pub created_at: &'a NaiveDateTime,
    pub updated_at: Option<&'a NaiveDateTime>,
    pub rome_uuid: &'a Vec<u8>,
    pub naf_uuid: &'a Vec<u8>,
}
