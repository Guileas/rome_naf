use crate::db::connection::Connection;
use crate::db::schema::keyword_nafs::naf_uuid;
use crate::db::schema::{keyword_nafs, keywords, nafs, specialtys};
use crate::models::specialty::{NewSpecialty, Specialty};
use crate::requests::NewKeywordSpecialtysRequest::NewKeywordSpecialtysRequest;
use crate::responses::resources::KeywordResource::KeywordResource;
use diesel::dsl::insert_into;
use diesel::sql_types::{Binary, Text};
use diesel::{self, prelude::*, sql_query};
use rocket::response::status::Accepted;
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::models::keyword::Keyword;
use crate::models::keyword::NewKeyword;
use crate::models::keyword::UpdateKeyword;
use crate::models::keyword_nafs::NewKeywordNaf;
use crate::models::naf::Naf;
use crate::requests::NewKeywordNafsRequest::NewKeywordNafsRequest;
use crate::requests::NewKeywordRequest::NewKeywordRequest;
use crate::responses::resources::CreationSuccessRessource::CreationSuccessRessource;
use crate::responses::resources::NafResource::NafResource;
use crate::responses::resources::ServerError::ServerError;
use crate::responses::resources::SuccessRessource::SuccessRessource;
use crate::route::v1::keyword::keyword::keywords::label;
use crate::serde::Serialize;
use rocket_okapi::JsonSchema;
use std::collections::HashMap;

#[derive(Debug, Serialize, JsonSchema)]
pub struct SearchResult {
    label: String,
    naf_codes: Vec<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct SearchResultWithNafId {
    label: String,
    naf_codes: Vec<String>,
    naf_ids: Vec<String>,
}

pub struct SearchNaf {
    keyword_id: Vec<u8>,
    label: String,
    naf_ids: Vec<Vec<u8>>,
}

#[derive(Debug, Serialize, JsonSchema, Queryable)]
pub struct SearchResultWithSpecialties {
    label: String,
    naf_codes: Vec<String>,
    specialties: Vec<MinimalSpecialty>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct MinimalNaf {
    code: String,
    id: Vec<u8>,
}
#[derive(Debug, Serialize, JsonSchema)]
pub struct MinimalSpecialty {
    id: String,
    name: String,
}

impl Queryable<(Binary, Text), diesel::mysql::Mysql> for MinimalSpecialty {
    type Row = (Vec<u8>, String);

    fn build(row: Self::Row) -> Self {
        let (id, name) = row;
        let default_uuid: Uuid = Uuid::parse_str("00000000000000000000000000000000").unwrap();
        let _uuid = match Uuid::from_slice(id.as_slice()) {
            Ok(_uuid) => _uuid,
            Err(_err) => default_uuid,
        };
        MinimalSpecialty {
            id: _uuid.to_string(),
            name,
        }
    }
}

#[openapi(tag = "Keyword", ignore = "connection")]
#[get("/v1/keyword")]
pub fn get_all_keyword(connection: Connection) -> Json<Vec<KeywordResource>> {
    let results = keywords::table
        .load::<Keyword>(&*connection)
        .expect("Error loading keyword");
    let mut _keyword = Vec::new();
    let default_uuid: Uuid = Uuid::parse_str("00000000000000000000000000000000").unwrap();

    for keyword in results {
        let _uuid = match Uuid::from_slice(keyword.uuid.as_slice()) {
            Ok(_uuid) => _uuid,
            Err(_err) => default_uuid,
        };

        _keyword.push(KeywordResource {
            uuid: _uuid.to_string(),
            label: keyword.label.to_string(),
        })
    }

    Json(_keyword)
}

#[openapi(tag = "Keyword", ignore = "connection")]
#[get("/v1/autocomplete/<search_term>")]
pub fn autocomplete_keyword_search(
    search_term: String,
    connection: Connection,
) -> Json<Vec<SearchResultWithNafId>> {
    use crate::models::keyword::Keyword;
    use crate::models::keyword_nafs::KeywordNaf;
    use crate::models::naf::Naf;

    let matching_keywords: Vec<SearchResultWithNafId> = keywords::table
        .inner_join(keyword_nafs::table.inner_join(nafs::table))
        .filter(keywords::label.like(format!("{}%", search_term)))
        .select((keywords::label, nafs::code, nafs::uuid))
        .load::<(String, String, Vec<u8>)>(&*connection)
        .expect("Could not load nafs")
        .into_iter()
        .fold(HashMap::new(), |mut map, (lab, naf_code, naf_id)| {
            let lab_clone = lab.clone();
            let default_uuid: Uuid = Uuid::parse_str("00000000000000000000000000000000").unwrap();
            let _uuid = match Uuid::from_slice(naf_id.as_slice()) {
                Ok(_uuid) => _uuid,
                Err(_err) => default_uuid,
            };

            map.entry(lab.clone())
                .or_insert_with(|| (Vec::new(), Vec::new())) // Tuple to store naf_codes and naf_uuids
                .0
                .push(naf_code);
            map.entry(lab_clone)
                .or_insert_with(|| (Vec::new(), Vec::new())) // Tuple to store naf_codes and naf_uuids
                .1
                .push(_uuid.to_string());
            map
        })
        .into_iter()
        .take(10)
        .map(|(keyword, (nafs, naf_id))| SearchResultWithNafId {
            label: keyword,
            naf_codes: nafs,
            naf_ids: naf_id,
        })
        .collect();

    Json(matching_keywords)
}

#[openapi(tag = "Keyword", ignore = "connection")]
#[get("/v1/keyword/<id>")]
pub fn get_keyword_by_id(connection: Connection, id: String) -> Json<Vec<KeywordResource>> {
    let _id = Uuid::parse_str(&id).unwrap();
    let keyword = keywords::table
        .filter(keywords::uuid.eq(&_id.as_bytes().to_vec()))
        .limit(1)
        .load::<Keyword>(&*connection)
        .expect("Error loading keyword");

    let default_uuid: Uuid = Uuid::parse_str("00000000000000000000000000000000").unwrap();
    let _uuid = match Uuid::from_slice(keyword[0].uuid.as_slice()) {
        Ok(_uuid) => _uuid,
        Err(_err) => default_uuid,
    };

    let mut _keywords = Vec::new();

    _keywords.push(KeywordResource {
        uuid: _uuid.to_string(),
        label: keyword[0].label.to_string(),
    });

    Json(_keywords)
}

#[openapi(tag = "Keyword", ignore = "connection")]
#[get("/v1/keyword_nafs/<id>")]
pub fn get_nafs_by_keyword(connection: Connection, id: String) -> Json<Vec<NafResource>> {
    let _id = Uuid::parse_str(&id).unwrap();
    let default_uuid: Uuid = Uuid::parse_str("00000000000000000000000000000000").unwrap();

    let results = keywords::table
        .inner_join(keyword_nafs::table.inner_join(nafs::table))
        .filter(keywords::uuid.eq(&_id.as_bytes().to_vec()))
        .select(nafs::all_columns)
        .load::<Naf>(&*connection)
        .expect("Could not load nafs");

    let mut _nafs = Vec::new();

    for naf in results {
        let _uuid = match Uuid::from_slice(naf.uuid.as_slice()) {
            Ok(_uuid) => _uuid,
            Err(_err) => default_uuid,
        };

        let _description = match naf.description {
            None => "",
            Some(ref x) => x,
        };

        _nafs.push(NafResource {
            uuid: _uuid.to_string(),
            code: naf.code.to_string(),
            label: naf.label.to_string(),
            description: Some(_description.to_string()),
        })
    }

    Json(_nafs)
}

#[openapi(tag = "Keyword", ignore = "_conn")]
#[post("/v1/keyword_nafs/delete", format = "application/json", data = "<request>")]
pub fn delete_specialty_by_id(
    _conn: Connection,
    request: Json<NewKeywordNafsRequest>,
) -> Json<SuccessRessource> {

    let _naf_id = Uuid::parse_str(&request.nafId).unwrap();

    let _keyword_uuid = Uuid::parse_str(&request.keywordId).unwrap();

    diesel::delete(
        keyword_nafs::table
            .filter(keyword_nafs::keyword_uuid.eq(&_keyword_uuid.as_bytes().to_vec()))
            .filter(keyword_nafs::naf_uuid.eq(&_naf_id.as_bytes().to_vec())),
    )
    //.find(&_id.as_bytes().to_vec()))
    .execute(&*_conn)
    .expect("Error deleting specialty");

    Json(SuccessRessource { success: true })
}

#[openapi(tag = "Keyword", ignore = "connection")]
#[post("/v1/keyword", format = "application/json", data = "<request>")]
pub fn insert_keyword(
    connection: Connection,
    request: Json<NewKeywordRequest>,
) -> Result<CreationSuccessRessource, ServerError<String>> {
    let keyword = keywords::table
        .filter(keywords::label.eq(&request.label.to_uppercase().to_string()))
        .limit(1)
        .load::<Keyword>(&*connection)
        .expect("Error loading keyword");

    if keyword.get(0).is_some() {
        let default_uuid: Uuid = Uuid::parse_str("00000000000000000000000000000000").unwrap();
        let _uuid = match Uuid::from_slice(keyword[0].uuid.as_slice()) {
            Ok(_uuid) => _uuid,
            Err(_err) => default_uuid,
        };

        return Ok(CreationSuccessRessource {
            existed: true,
            message: "Keyword exists".to_string(),
            uuid: _uuid.to_string(),
        });
    }

    let new_uuid = Uuid::new_v4();
    let new_keyword = NewKeyword {
        uuid: &new_uuid.as_bytes().to_vec(),
        created_at: &chrono::Local::now().naive_utc(),
        updated_at: None,
        label: &request.label.to_uppercase(),
    };

    match diesel::insert_into(keywords::table)
        .values(&new_keyword)
        .execute(&*connection)
    {
        Ok(_) => Ok(CreationSuccessRessource {
            existed: false,
            message: "Keyword".to_string(),
            uuid: new_uuid.to_string(),
        }),
        Err(_) => Err(ServerError("Unable to create the keyword".to_string())),
    }
}

#[openapi(tag = "Keyword", ignore = "connection")]
#[post("/v1/keyword_nafs", format = "application/json", data = "<request>")]
pub fn link_keyword_to_nafs(
    connection: Connection,
    request: Json<NewKeywordNafsRequest>,
) -> Result<CreationSuccessRessource, ServerError<String>> {
    let new_uuid = Uuid::new_v4();

    let naf_id = Uuid::parse_str(&request.nafId).unwrap();

    let keyword_uuid = Uuid::parse_str(&request.keywordId).unwrap();

    let new_keyword_nafs = NewKeywordNaf {
        uuid: &new_uuid.as_bytes().to_vec(),
        created_at: &chrono::Local::now().naive_utc(),
        updated_at: None,
        keyword_uuid: &keyword_uuid.as_bytes().to_vec(),
        naf_uuid: &naf_id.as_bytes().to_vec(),
    };

    match diesel::insert_into(keyword_nafs::table)
        .values(&new_keyword_nafs)
        .execute(&*connection)
    {
        Ok(_) => Ok(CreationSuccessRessource {
            existed: false,
            message: "Keyword_Naf".to_string(),
            uuid: new_uuid.to_string(),
        }),
        Err(_) => Err(ServerError(
            "Unable to create the rome_nafs element".to_string(),
        )),
    }
}

#[openapi(tag = "Keyword", ignore = "connection")]
#[put("/v1/keyword/<id>", format = "application/json", data = "<keyword>")]
pub fn update_keyword_by_id(
    id: String,
    keyword: Json<NewKeywordRequest>,
    connection: Connection,
) -> Result<Accepted<Json<SuccessRessource>>, ServerError<String>> {
    let _id = Uuid::parse_str(&id).unwrap();

    let _keyword = UpdateKeyword {
        uuid: _id.as_bytes().to_vec(),
        updated_at: Some(chrono::Local::now().naive_utc()),
        label: keyword.label.to_string(),
    };

    match diesel::update(keywords::table.find(&_id.as_bytes().to_vec()))
        .set(_keyword)
        .execute(&*connection)
    {
        Ok(_) => Ok(Accepted::<Json<SuccessRessource>>(Some(Json(
            SuccessRessource { success: true },
        )))),
        Err(_) => Err(ServerError("Unable to update the keyword".to_string())),
    }
}

#[openapi(tag = "Keyword", ignore = "_conn")]
#[delete("/v1/keyword/<id>")]
pub fn delete_keyword_by_id(_conn: Connection, id: String) -> Json<SuccessRessource> {
    let _id = Uuid::parse_str(&id).unwrap();

    diesel::delete(keywords::table.filter(keywords::uuid.eq(&_id.as_bytes().to_vec())))
        //.find(&_id.as_bytes().to_vec()))
        .execute(&*_conn)
        .expect("Error deleting keyword");

    Json(SuccessRessource { success: true })
}
