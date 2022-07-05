use diesel::mysql::MysqlConnection;
use rocket::request::{self, FromRequest};
use rocket::http::Status;
use rocket::outcome::{try_outcome, Outcome};
use rocket::{Request, State};
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager };
use std::ops::Deref;
use dotenv::dotenv;
use std::env;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::gen::OpenApiGenerator;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
//static DATABASE_URL: &'static str = env!("DATABASE_URL");

pub fn  connect() -> MysqlPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub struct Connection(pub PooledConnection<ConnectionManager<MysqlConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Connection {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let pool = try_outcome!(request.guard::<&State<MysqlPool>>().await);
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}


/// !INFO: https://github.com/GREsau/okapi#faq
/// use the fix error : "the trait bound `db::connection::Connection: OpenApiFromRequest<'_>` is not satisfied"
impl<'r> OpenApiFromRequest<'r> for Connection {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

// For the convenience of using an &Connection as an &MysqlConnection.
impl Deref for Connection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
