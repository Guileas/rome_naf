use crate::db::connection::Connection;
use crate::db::schema::romes;
use rocket::serde::json::Json;
use rocket::response::status::Accepted;
use diesel::dsl::insert_into;
use diesel::{self, prelude::*};
use uuid::Uuid;

use crate::responses::resources::ServerError::ServerError;
use crate::responses::resources::SuccessRessource::SuccessRessource;
use crate::models::rome::UpdateRome;
use crate::models::rome::NewRome;
use crate::requests::NewRomeRequest::NewRomeRequest;
use crate::models::rome::Rome;
use crate::responses::resources::RomeResource::RomeResource;


#[openapi(tag = "Rome")]
#[get("/v1/rome")]
pub fn get_all_rome(connection: Connection) -> Json<Vec<RomeResource>> {

    let results = romes::table.load::<Rome>(&*connection).expect("Error loading romes");
    let mut _romes = Vec::new();
    let default_uuid: Uuid = Uuid::parse_str("00000000000000000000000000000000").unwrap();


    for rome in results {

        let _uuid = match Uuid::from_slice(rome.uuid.as_slice()) {
            Ok(_uuid) => _uuid,
            Err(_err) => default_uuid,
        };

        let _description = match rome.description{
            None => "",
            Some(ref x) => x,
        };

        _romes.push(RomeResource{
            uuid: _uuid.to_string(),
            code: rome.code.to_string(),
            label: rome.label.to_string(),
            description: Some(_description.to_string()),
        })
    }

    Json(_romes)
}

#[openapi(tag = "Rome")]
#[get("/v1/rome/<id>")]
pub fn get_rome_by_id(connection: Connection, id: String) -> Json<Vec<RomeResource>> {

    let _id = Uuid::parse_str(&id).unwrap();
    let rome = romes::table
        .filter(romes::uuid.eq(&_id.as_bytes().to_vec()))
        .limit(1)
        .load::<Rome>(&*connection)
        .expect("Error loading rome");

    let default_uuid: Uuid = Uuid::parse_str("00000000000000000000000000000000").unwrap();
    let _uuid = match Uuid::from_slice(rome[0].uuid.as_slice()) {
        Ok(_uuid) => _uuid,
        Err(_err) => default_uuid,
    };

    let _description = match rome[0].description{
        None => "",
        Some(ref x) => x,
    };

    let mut _romes = Vec::new();

    _romes.push(RomeResource{
        uuid: _uuid.to_string(),
        code: rome[0].code.to_string(),
        label: rome[0].label.to_string(),
        description: Some(_description.to_string()),
    });

    Json(_romes)
}


#[openapi(tag = "Rome")]
#[post("/v1/rome", format = "application/json", data = "<request>")]
pub fn insert_rome(connection: Connection, request: Json<NewRomeRequest>)-> Result<Accepted<Json<SuccessRessource>>, ServerError<String>> {

    let _description= match request.description{
        None => None,
        Some(ref x) => Some(x),
    };

    let new_uuid = Uuid::new_v4();
    let new_rome = NewRome {
        uuid: &new_uuid.as_bytes().to_vec(),
        created_at: &chrono::Local::now().naive_utc(),
        updated_at: None,
        code: &request.code,
        label: &request.label,
        description: _description
    };

    match diesel::insert_into(romes::table).values(&new_rome).execute(&*connection) {
        Ok(_) => Ok(Accepted::<Json<SuccessRessource>>(Some(Json(
            SuccessRessource { success: true },
        )))),
        Err(_) => Err(ServerError("Unable to create the rome".to_string())),
    }
}

#[openapi(tag = "Rome")]
#[put("/v1/rome/<id>", format = "application/json", data = "<rome>")]
pub fn update_rome_by_id(id: String, rome: Json<NewRomeRequest>,  connection: Connection) -> Result<Accepted<Json<SuccessRessource>>, ServerError<String>> {
    let _id = Uuid::parse_str(&id).unwrap();

    let _description= match rome.description{
        None => None,
        Some(ref x) => Some(String::from(x)),
    };

    let _rome = UpdateRome {
        uuid: _id.as_bytes().to_vec(),
        updated_at: Some(chrono::Local::now().naive_utc()),
        code: rome.code.to_string(),
        label: rome.label.to_string(),
        description: _description,
    };

    match diesel::update(romes::table
        .find(&_id.as_bytes().to_vec()))
        .set(_rome)
        .execute(&*connection) {
            Ok(_) => Ok(Accepted::<Json<SuccessRessource>>(Some(Json(
                SuccessRessource { success: true },
            )))),
            Err(_) => Err(ServerError("Unable to update the rome".to_string())),
        }
}


#[openapi(tag = "Rome")]
#[delete("/v1/rome/<id>")]
pub fn delete_rome_by_id(_conn: Connection, id: String) -> Json<SuccessRessource> {
    let _id = Uuid::parse_str(&id).unwrap();

    diesel::delete(
            romes::table.filter(
                romes::uuid.eq(&_id.as_bytes().to_vec())
            )
        )
        //.find(&_id.as_bytes().to_vec()))
        .execute(&*_conn)
        .expect("Error deleting rome");

    Json(SuccessRessource{ success: true })
}
