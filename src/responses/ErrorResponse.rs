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
pub struct ErrorResponse {
    message: String,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ServerError<String> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        Response::build_from(
            Json(ErrorResponse {
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
        let schema = gen.json_schema::<ErrorResponse>();
        add_schema_response(&mut responses, 500, "application/json", schema)
            .expect("Add response failed");
        Ok(responses)
    }
}
