use diesel::QueryDsl;
use rocket::response::status::Accepted;
use rocket::serde::json::Json;
use uuid::Uuid;
use crate::db::schema::specialtys;
use crate::models::specialty::{Specialty, self, NewSpecialty, UpdateSpecialty};
use crate::requests::NewSpecialtysRequest::NewSpecialtyRequest;
use crate::responses::resources::ServerError::ServerError;
use crate::responses::resources::CreationSuccessRessource::CreationSuccessRessource;
use crate::responses::resources::SpecialtyResource::SpecialtyResource;
use crate::responses::resources::SuccessRessource::SuccessRessource;
use crate::{db::{connection::Connection}};
use diesel::prelude::*;

#[openapi(tag = "Specialty", ignore = "connection")]
#[get("/v1/specialty_naf/<id>")]
pub fn get_specialities_by_naf(connection: Connection, id: String) -> Json<Vec<SpecialtyResource>>{

    let _id = Uuid::parse_str(&id).unwrap();
    let default_uuid: Uuid = Uuid::parse_str("00000000000000000000000000000000").unwrap();

    let results = specialtys::table
        .filter(specialtys::naf_id.eq(&_id.as_bytes().to_vec()))
        .load::<Specialty>(&*connection)
        .expect("Error loading speciltys");

    let mut _specialty:Vec<SpecialtyResource> = Vec::new();

    for specialty in results {

        let _uuid = match Uuid::from_slice(specialty.uuid.as_slice()) {
            Ok(_uuid) => _uuid,
            Err(_err) => default_uuid,
        };

        let _description = match specialty.description{
            None => "",
            Some(ref x) => x,
        };

        _specialty.push(SpecialtyResource{
            uuid: _uuid.to_string(),
            label: specialty.label.to_string(),
            description: Some(_description.to_string()),
            keyword_id: id.to_string(),
        })
    }

    Json(_specialty)
}


#[openapi(tag = "Specialty", ignore = "connection")]
#[post("/v1/specialty", format = "application/json", data = "<request>")]
pub fn insert_specialty(connection: Connection, request: Json<NewSpecialtyRequest>)-> Result<CreationSuccessRessource, ServerError<String>> {
    let naf_id = Uuid::parse_str(&request.naf_id).unwrap();

    let specialty = specialtys::table
    .filter(specialtys::label.eq(&request.label.to_string()))
    .limit(1)
    .load::<Specialty>(&*connection)
    .expect("Error loading specialty");

    if specialty.get(0).is_some(){
        let default_uuid: Uuid = Uuid::parse_str("00000000000000000000000000000000").unwrap();
        let _uuid = match Uuid::from_slice(specialty[0].uuid.as_slice()) {
            Ok(_uuid) => _uuid,
            Err(_err) => default_uuid,
        };

        return Ok(CreationSuccessRessource { existed: true, message: "Specialty already exists".to_string(), uuid: _uuid.to_string() })
    }

    let _description= match request.description{
        None => None,
        Some(ref x) => Some(x),
    };

    let new_uuid = Uuid::new_v4();
    let new_specialty = NewSpecialty {
        uuid: &new_uuid.as_bytes().to_vec(),
        created_at: &chrono::Local::now().naive_utc(),
        updated_at: None,
        label: &request.label,
        description: _description,
        naf_id: &naf_id.as_bytes().to_vec()
    };

    match diesel::insert_into(specialtys::table).values(&new_specialty).execute(&*connection) {
        Ok(_) => Ok(CreationSuccessRessource { existed: false, message: "Specialty created".to_string(), uuid: new_uuid.to_string() }),
        Err(x) => {
            Err(ServerError("Unable to create the specialty".to_string()))  },
    }
}

#[openapi(tag = "Specialty", ignore = "connection")]
#[put("/v1/specialty/<id>", format = "application/json", data = "<specialty>")]
pub fn update_specialty_by_id(id: String, specialty: Json<NewSpecialtyRequest>,  connection: Connection) -> Result<Accepted<Json<SuccessRessource>>, ServerError<String>> {
    let _id = Uuid::parse_str(&id).unwrap();
    let naf_id = Uuid::parse_str(&specialty.naf_id).unwrap();
    let _description= match specialty.description{
        None => None,
        Some(ref x) => Some(String::from(x)),
    };

    let _updated_specialty = UpdateSpecialty {
        uuid: _id.as_bytes().to_vec(),
        updated_at: Some(chrono::Local::now().naive_utc()),
        naf_id: naf_id.as_bytes().to_vec(),
        label: specialty.label.to_string(),
        description: _description,
    };

    match diesel::update(specialtys::table
        .find(&_id.as_bytes().to_vec()))
        .set(_updated_specialty)
        .execute(&*connection) {
            Ok(_) => Ok(Accepted::<Json<SuccessRessource>>(Some(Json(
                SuccessRessource { success: true },
            )))),
            Err(_) => Err(ServerError("Unable to update the specialty".to_string())),
        }
}


#[openapi(tag = "Specialty", ignore = "_conn")]
#[delete("/v1/specialty/<id>")]
pub fn delete_specialty_by_id(_conn: Connection, id: String) -> Json<SuccessRessource> {
    let _id = Uuid::parse_str(&id).unwrap();

    diesel::delete(
        specialtys::table.filter(
            specialtys::uuid.eq(&_id.as_bytes().to_vec())
            )
        )
        //.find(&_id.as_bytes().to_vec()))
        .execute(&*_conn)
        .expect("Error deleting specialty");

    Json(SuccessRessource{ success: true })
}
