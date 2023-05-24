use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::Json;
use schemars::JsonSchema;
use serde::Serialize;

use rocket_okapi::okapi::openapi3::Responses;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::util::add_schema_response;
use rocket_okapi::OpenApiError;

#[derive(Debug, Clone, PartialEq)]
pub struct ServerError<String>(pub String);

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct InternalServerError {
    #[schemars(example = "example_code")]
    code: u16,
    #[schemars(example = "example_message")]
    message: String,
}

pub fn example_code() -> &'static u16 {
    &500
}

pub fn example_message() -> &'static str {
    "Something failed"
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ServerError<String> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        Response::build_from(
            Json(InternalServerError {
                code: 500,
                message: self.0,
            })
            .respond_to(req)?,
        )
        .header(ContentType::JSON)
        .status(Status::InternalServerError)
        .ok()
    }
}

impl OpenApiResponderInner for ServerError<String> {
    fn responses(gen: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        let mut responses = Responses::default();
        let schema = gen.json_schema::<InternalServerError>();
        add_schema_response(&mut responses, 500, "application/json", schema)
            .expect("Add response failed");
        Ok(responses)
    }
}
