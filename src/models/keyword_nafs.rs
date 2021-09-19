use chrono::NaiveDateTime;
use crate::db::schema::keyword_nafs;
use crate::models::keyword::Keyword;
use crate::models::naf::Naf;

#[derive(AsChangeset, Queryable, Identifiable)]
#[primary_key(uuid)]
#[column_name(uuid)]
pub struct KeywordNaf {
    pub uuid: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub keyword_uuid: Vec<u8>,
    pub naf_uuid: Vec<u8>
}

#[derive(Insertable)]
#[table_name = "keyword_nafs"]
pub struct NewKeywordNaf<'a> {
    pub uuid: &'a Vec<u8>,
    pub created_at: &'a NaiveDateTime,
    pub updated_at: Option<&'a NaiveDateTime>,
    pub keyword_uuid: &'a Vec<u8>,
    pub naf_uuid: &'a Vec<u8>,
}
