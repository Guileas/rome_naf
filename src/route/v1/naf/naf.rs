use crate::responses::resources::KeywordResource::KeywordResource;
use crate::db::connection::Connection;
use crate::db::schema::nafs;
use rocket::serde::json::Json;
use rocket::response::status::Accepted;
use diesel::dsl::insert_into;
use diesel::{self, prelude::*};
use uuid::Uuid;
use crate::models::naf::NewNaf;

use crate::responses::resources::SuccessRessource::SuccessRessource;
use crate::responses::resources::CreationSuccessRessource::CreationSuccessRessource;
use crate::responses::resources::NafResource::NafResource;
use crate::models::naf::Naf;
use crate::responses::resources::ServerError::ServerError;
use crate::requests::NewNafRequest::NewNafRequest;
use crate::models::naf::UpdateNaf;

#[openapi(tag = "Naf")]
#[get("/v1/naf")]
pub fn get_all_naf(connection: Connection) -> Json<Vec<NafResource>> {


    let results = nafs::table.load::<Naf>(&*connection).expect("Error loading nafs");
    let mut _nafs = Vec::new();
    let default_uuid: Uuid = Uuid::parse_str("00000000000000000000000000000000").unwrap();


    for naf in results {

        let _uuid = match Uuid::from_slice(naf.uuid.as_slice()) {
            Ok(_uuid) => _uuid,
            Err(_err) => default_uuid,
        };

        let _description = match naf.description{
            None => "",
            Some(ref x) => x,
        };

        _nafs.push(NafResource{
            uuid: _uuid.to_string(),
            code: naf.code.to_string(),
            label: naf.label.to_string(),
            description: Some(_description.to_string()),
        })
    }

    Json(_nafs)
}

#[openapi(tag = "Naf")]
#[get("/v1/naf/<id>")]
pub fn get_naf_by_id(connection: Connection, id: String) -> Json<Vec<NafResource>> {

    let _id = Uuid::parse_str(&id).unwrap();
    let naf = nafs::table
        .filter(nafs::uuid.eq(&_id.as_bytes().to_vec()))
        .limit(1)
        .load::<Naf>(&*connection)
        .expect("Error loading naf");

    let default_uuid: Uuid = Uuid::parse_str("00000000000000000000000000000000").unwrap();
    let _uuid = match Uuid::from_slice(naf[0].uuid.as_slice()) {
        Ok(_uuid) => _uuid,
        Err(_err) => default_uuid,
    };

    let _description = match naf[0].description{
        None => "",
        Some(ref x) => x,
    };

    let mut _nafs = Vec::new();

    _nafs.push(NafResource{
        uuid: _uuid.to_string(),
        code: naf[0].code.to_string(),
        label: naf[0].label.to_string(),
        description: Some(_description.to_string()),
    });

    Json(_nafs)
}


#[openapi(tag = "Naf")]
#[post("/v1/naf", format = "application/json", data = "<request>")]
pub fn insert_naf(connection: Connection, request: Json<NewNafRequest>)-> Result<Accepted<Json<CreationSuccessRessource>>, ServerError<String>> {

    let _description= match request.description{
        None => None,
        Some(ref x) => Some(x),
    };

    let new_uuid = Uuid::new_v4();
    let new_naf = NewNaf {
        uuid: &new_uuid.as_bytes().to_vec(),
        created_at: &chrono::Local::now().naive_utc(),
        updated_at: None,
        code: &request.code,
        label: &request.label,
        description: _description
    };

    match diesel::insert_into(nafs::table).values(&new_naf).execute(&*connection) {
        Ok(_) => Ok(Accepted::<Json<CreationSuccessRessource>>(Some(Json(
            CreationSuccessRessource { success: true, uuid: new_uuid.to_string() },
        )))),
        Err(_) => Err(ServerError("Unable to create the naf".to_string())),
    }
}

#[openapi(tag = "Naf")]
#[put("/v1/naf/<id>", format = "application/json", data = "<naf>")]
pub fn update_naf_by_id(id: String, naf: Json<NewNafRequest>,  connection: Connection) -> Result<Accepted<Json<CreationSuccessRessource>>, ServerError<String>> {
    let _id = Uuid::parse_str(&id).unwrap();

    let _description= match naf.description{
        None => None,
        Some(ref x) => Some(String::from(x)),
    };

    let _naf = UpdateNaf {
        uuid: _id.as_bytes().to_vec(),
        updated_at: Some(chrono::Local::now().naive_utc()),
        code: naf.code.to_string(),
        label: naf.label.to_string(),
        description: _description,
    };

    match diesel::update(nafs::table
        .find(&_id.as_bytes().to_vec()))
        .set(_naf)
        .execute(&*connection) {
            Ok(_) => Ok(Accepted::<Json<CreationSuccessRessource>>(Some(Json(
                CreationSuccessRessource { success: true, uuid: _id.to_string() },
            )))),
            Err(_) => Err(ServerError("Unable to update the naf".to_string())),
        }
}


#[openapi(tag = "Naf")]
#[delete("/v1/naf/<id>")]
pub fn delete_naf_by_id(_conn: Connection, id: String) -> Json<SuccessRessource> {
    let _id = Uuid::parse_str(&id).unwrap();

    diesel::delete(
            nafs::table.filter(
                nafs::uuid.eq(&_id.as_bytes().to_vec())
            )
        )
        //.find(&_id.as_bytes().to_vec()))
        .execute(&*_conn)
        .expect("Error deleting naf");

    Json(SuccessRessource{ success: true })
}
