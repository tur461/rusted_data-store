use std::env;
use diesel::r2d2;
use rocket::State;
use rocket::http::Status;
use diesel::pg::PgConnection;
use rocket::request::Outcome;
use std::ops::{Deref, DerefMut};
use diesel::r2d2::ConnectionManager;
use rocket::request::{Request, FromRequest};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).expect("db pool")
}

fn database_url() -> String {
    env::var("DATABASE_URL").expect("database url must be set in env")
}

pub struct DbCon(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbCon {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<DbCon, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(con) => Outcome::Success(DbCon(con)),
            Err(e) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    } 
}

impl Deref for DbCon {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DbCon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}