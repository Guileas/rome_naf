use crate::db::connection::Connection;
use crate::db::schema::specialtys::naf_id;
use crate::db::schema::{nafs, specialtys};
use crate::models::naf::Naf;
use crate::models::naf::NewNaf;
use crate::models::naf::UpdateNaf;
use crate::models::specialty::Specialty;
use crate::requests::NewNafRequest::NewNafRequest;
use crate::responses::resources::CreationSuccessRessource::CreationSuccessRessource;
use crate::responses::resources::KeywordResource::KeywordResource;
use crate::responses::resources::NafResource::NafResource;
use crate::responses::resources::ServerError::ServerError;
use crate::responses::resources::SpecialtyResource::SpecialtyResource;
use crate::responses::resources::SuccessRessource::SuccessRessource;
use crate::route::v1::keyword::keyword::MinimalSpecialty;
use crate::route::v1::specialty;
use diesel::dsl::insert_into;
use diesel::{self, prelude::*};
use rocket::http::hyper::request;
use rocket::response::status::Accepted;
use rocket::serde::json::Json;
use uuid::Uuid;

#[openapi(tag = "Naf", ignore = "connection")]
#[get("/v1/naf")]
pub fn get_all_naf(connection: Connection) -> Json<Vec<NafResource>> {
    let results = nafs::table
        .load::<Naf>(&*connection)
        .expect("Error loading nafs");
    let mut _nafs = Vec::new();
    let default_uuid: Uuid = Uuid::parse_str("00000000000000000000000000000000").unwrap();

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

#[openapi(tag = "Naf", ignore = "connection")]
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

    let _description = match naf[0].description {
        None => "",
        Some(ref x) => x,
    };

    let mut _nafs = Vec::new();

    _nafs.push(NafResource {
        uuid: _uuid.to_string(),
        code: naf[0].code.to_string(),
        label: naf[0].label.to_string(),
        description: Some(_description.to_string()),
    });

    Json(_nafs)
}

#[openapi(tag = "Naf", ignore = "connection")]
#[get("/v1/naf/specialties/<id>")]
pub fn get_naf_specialty_list(
    connection: Connection,
    id: String,
) -> Json<Vec<MinimalSpecialty>> {

    let _id = Uuid::parse_str(&id).unwrap();

    let _specialties = specialtys::table
    .filter(specialtys::naf_id.eq(&_id.as_bytes().to_vec()))
    .select((specialtys::uuid, specialtys::label))
    .load::<MinimalSpecialty>(&*connection)
    .expect("Error loading specialties");

    Json(_specialties)
}

#[openapi(tag = "Naf", ignore = "connection")]
#[post("/v1/naf", format = "application/json", data = "<request>")]
pub fn insert_naf(
    connection: Connection,
    request: Json<NewNafRequest>,
) -> Result<CreationSuccessRessource, ServerError<String>> {
    let naf = nafs::table
        .filter(nafs::code.eq(&request.code.to_string()))
        .limit(1)
        .load::<Naf>(&*connection)
        .expect("Error loading naf");

    if naf.get(0).is_some() {
        let default_uuid: Uuid = Uuid::parse_str("00000000000000000000000000000000").unwrap();
        let _uuid = match Uuid::from_slice(naf[0].uuid.as_slice()) {
            Ok(_uuid) => _uuid,
            Err(_err) => default_uuid,
        };

        return Ok(CreationSuccessRessource {
            existed: true,
            message: "Naf exists".to_string(),
            uuid: _uuid.to_string(),
        });
    }

    let _description = match request.description {
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
        description: _description,
    };

    match diesel::insert_into(nafs::table)
        .values(&new_naf)
        .execute(&*connection)
    {
        Ok(_) => Ok(CreationSuccessRessource {
            existed: false,
            message: "Naf".to_string(),
            uuid: new_uuid.to_string(),
        }),
        Err(_) => Err(ServerError("Unable to create the naf".to_string())),
    }
}

#[openapi(tag = "Naf", ignore = "connection")]
#[put("/v1/naf/<id>", format = "application/json", data = "<naf>")]
pub fn update_naf_by_id(
    id: String,
    naf: Json<NewNafRequest>,
    connection: Connection,
) -> Result<CreationSuccessRessource, ServerError<String>> {
    let _id = Uuid::parse_str(&id).unwrap();

    let _description = match naf.description {
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

    match diesel::update(nafs::table.find(&_id.as_bytes().to_vec()))
        .set(_naf)
        .execute(&*connection)
    {
        Ok(_) => Ok(CreationSuccessRessource {
            existed: false,
            message: "Update Naf".to_string(),
            uuid: _id.to_string(),
        }),
        Err(_) => Err(ServerError("Unable to update the naf".to_string())),
    }
}

#[openapi(tag = "Naf", ignore = "connection")]
#[post("/v1/naf/specialties", format = "application/json", data = "<request>")]
pub fn get_nafs_specialtys(
    connection: Connection,
    request: Json<Vec<String>>,
) -> Json<Vec<MinimalSpecialty>> {

    let ids: Vec<Vec<u8>> = request
        .iter()
        .map(|s| {
            let a = Uuid::parse_str(&s).expect("Invalid UUID");
            a.as_bytes().to_vec()
        })
        .collect();

    let _specialties = specialtys::table
    .filter(specialtys::naf_id.eq_any(ids))
    .select((specialtys::uuid, specialtys::label))
    .load::<MinimalSpecialty>(&*connection)
    .expect("Error loading specialties");

    Json(_specialties)
}

#[openapi(tag = "Naf", ignore = "_conn")]
#[delete("/v1/naf/<id>")]
pub fn delete_naf_by_id(_conn: Connection, id: String) -> Json<SuccessRessource> {
    let _id = Uuid::parse_str(&id).unwrap();

    diesel::delete(nafs::table.filter(nafs::uuid.eq(&_id.as_bytes().to_vec())))
        //.find(&_id.as_bytes().to_vec()))
        .execute(&*_conn)
        .expect("Error deleting naf");

    Json(SuccessRessource { success: true })
}
