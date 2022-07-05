#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket_okapi;

#[cfg(test)] mod tests;

mod db;
mod models;
mod requests;
mod responses;
mod route;
mod fairing;

use rocket::*;
use rocket_okapi::{ openapi, openapi_get_routes, swagger_ui::{make_swagger_ui, SwaggerUIConfig}};
use db::connection::connect;
use fairing::cors::CORS;

#[openapi]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn build_rocket() -> Rocket<Build> {
    rocket::build()
    .manage(connect())
    .mount(
        "/",
        openapi_get_routes![
            index,
            route::v1::naf::naf::insert_naf,
            route::v1::naf::naf::get_all_naf,
            route::v1::naf::naf::get_naf_by_id,
            route::v1::naf::naf::update_naf_by_id,
            route::v1::naf::naf::delete_naf_by_id,
            //Keywords
            route::v1::keyword::keyword::insert_keyword,
            route::v1::keyword::keyword::get_all_keyword,
            route::v1::keyword::keyword::get_keyword_by_id,
            route::v1::keyword::keyword::update_keyword_by_id,
            route::v1::keyword::keyword::delete_keyword_by_id,
            route::v1::keyword::keyword::link_keyword_to_nafs,
            route::v1::keyword::keyword::get_nafs_by_keyword,
            //Rome
            route::v1::rome::rome::insert_rome,
            route::v1::rome::rome::get_all_rome,
            route::v1::rome::rome::get_rome_by_id,
            route::v1::rome::rome::update_rome_by_id,
            route::v1::rome::rome::delete_rome_by_id,
            route::v1::rome::rome::link_rome_to_nafs,
            route::v1::rome::rome::get_nafs_by_rome,
        ],
    )
    .mount(
        "/swagger-ui/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../openapi.json".to_owned(),
            ..Default::default()
        }),
    ).attach(CORS)
}

#[launch]
fn rocket() -> _ {
    build_rocket()
}
