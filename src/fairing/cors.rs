use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use std::io::Cursor;
use rocket::http::{Header, ContentType, Method, Status};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON) {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PUT, DELETE, PATCH, OPTIONS"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }

        if request.method() == Method::Options {
            let string = "";
            response.set_header(ContentType::Plain);
            response.set_sized_body(string.len(),Cursor::new(string));
            response.set_status(Status::NoContent);
        }
    }
}
