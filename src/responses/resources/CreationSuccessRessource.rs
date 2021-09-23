use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::Json;
use schemars::JsonSchema;
use serde::Serialize;

use okapi::openapi3::Responses;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::util::add_schema_response;
use rocket_okapi::OpenApiError;


#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreationSuccessRessource {
    pub existed: bool,
    pub message: String,
    pub uuid: String
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct InternalSuccessResponse {
    #[schemars(example = "example_message")]
    message: String,
    uuid: String
}

pub fn example_message() -> &'static str {
    "Something happend"
}

impl<'r, 'o: 'r> Responder<'r, 'o> for CreationSuccessRessource {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {

        Response::build_from(
            Json(InternalSuccessResponse {
                message: self.message,
                uuid: self.uuid
            })
            .respond_to(req)?,
        )
        .header(ContentType::JSON)
        .status(
            match self.existed
            {
                true => Status::Ok,
                false => Status::Created
            }
        )
        .ok()
    }
}

impl OpenApiResponderInner for CreationSuccessRessource {
    fn responses(gen: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        let mut responses = Responses::default();
        let schema = gen.json_schema::<InternalSuccessResponse>();
        let schema2 = gen.json_schema::<InternalSuccessResponse>();
        add_schema_response(&mut responses, 201, "application/json", schema)
            .expect("Add response created");
        add_schema_response(&mut responses, 200, "application/json", schema2)
            .expect("Add response exist");
        Ok(responses)
    }
}
