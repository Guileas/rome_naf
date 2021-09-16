use crate::responses::resources::KeywordResource::KeywordResource;
use crate::db::connection::Connection;
use crate::db::schema::keywords;
use rocket::serde::json::Json;
use rocket::response::status::Accepted;
use diesel::dsl::insert_into;
use diesel::{self, prelude::*};
use uuid::Uuid;

use crate::responses::resources::ServerError::ServerError;
use crate::responses::resources::SuccessRessource::SuccessRessource;
use crate::models::keyword::UpdateKeyword;
use crate::models::keyword::NewKeyword;
use crate::requests::NewKeywordRequest::NewKeywordRequest;
use crate::models::keyword::Keyword;

#[openapi(tag = "Keyword")]
#[get("/v1/keyword")]
pub fn get_all_keyword(connection: Connection) -> Json<Vec<KeywordResource>> {


    let results = keywords::table.load::<Keyword>(&*connection).expect("Error loading keyword");
    let mut _keyword = Vec::new();
    let default_uuid: Uuid = Uuid::parse_str("00000000000000000000000000000000").unwrap();


    for keyword in results {

        let _uuid = match Uuid::from_slice(keyword.uuid.as_slice()) {
            Ok(_uuid) => _uuid,
            Err(_err) => default_uuid,
        };

        _keyword.push(KeywordResource{
            uuid: _uuid.to_string(),
            label: keyword.label.to_string(),
        })
    }

    Json(_keyword)
}

#[openapi(tag = "Keyword")]
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

    _keywords.push(KeywordResource{
        uuid: _uuid.to_string(),
        label: keyword[0].label.to_string(),
    });

    Json(_keywords)
}


#[openapi(tag = "Keyword")]
#[post("/v1/keyword", format = "application/json", data = "<request>")]
pub fn insert_keyword(connection: Connection, request: Json<NewKeywordRequest>)-> Result<Accepted<Json<SuccessRessource>>, ServerError<String>> {

    let new_uuid = Uuid::new_v4();
    let new_keyword = NewKeyword {
        uuid: &new_uuid.as_bytes().to_vec(),
        created_at: &chrono::Local::now().naive_utc(),
        updated_at: None,
        label: &request.label
    };

    match diesel::insert_into(keywords::table).values(&new_keyword).execute(&*connection) {
        Ok(_) => Ok(Accepted::<Json<SuccessRessource>>(Some(Json(
            SuccessRessource { success: true },
        )))),
        Err(_) => Err(ServerError("Unable to create the keyword".to_string())),
    }
}

#[openapi(tag = "Keyword")]
#[put("/v1/keyword/<id>", format = "application/json", data = "<keyword>")]
pub fn update_keyword_by_id(id: String, keyword: Json<NewKeywordRequest>,  connection: Connection) -> Result<Accepted<Json<SuccessRessource>>, ServerError<String>> {
    let _id = Uuid::parse_str(&id).unwrap();

    let _keyword = UpdateKeyword {
        uuid: _id.as_bytes().to_vec(),
        updated_at: Some(chrono::Local::now().naive_utc()),
        label: keyword.label.to_string(),
    };

    match diesel::update(keywords::table
        .find(&_id.as_bytes().to_vec()))
        .set(_keyword)
        .execute(&*connection) {
            Ok(_) => Ok(Accepted::<Json<SuccessRessource>>(Some(Json(
                SuccessRessource { success: true },
            )))),
            Err(_) => Err(ServerError("Unable to update the keyword".to_string())),
        }
}


#[openapi(tag = "Keyword")]
#[delete("/v1/keyword/<id>")]
pub fn delete_keyword_by_id(_conn: Connection, id: String) -> Json<SuccessRessource> {
    let _id = Uuid::parse_str(&id).unwrap();

    diesel::delete(
            keywords::table.filter(
                keywords::uuid.eq(&_id.as_bytes().to_vec())
            )
        )
        //.find(&_id.as_bytes().to_vec()))
        .execute(&*_conn)
        .expect("Error deleting keyword");

    Json(SuccessRessource{ success: true })
}