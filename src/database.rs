use std::ops::Deref;

use mongodb::{self, db::Database, Client, ThreadedClient};
use rocket::{
    request::{self, FromRequest},
    Outcome, Request, State,
};

#[derive(Clone)]
pub struct MongoDatabase(Database);

impl MongoDatabase {
    pub fn connect(uri: &str, db_name: &str) -> mongodb::Result<MongoDatabase> {
        let client = Client::with_uri(uri)?;
        Ok(MongoDatabase(client.db(db_name)))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for MongoDatabase {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<MongoDatabase, Self::Error> {
        let db = request.guard::<State<MongoDatabase>>()?;
        // The Database type is actually just an Arc so a clone is cheap
        Outcome::Success(db.clone())
    }
}

// For the convenience of using an &MongoDatabase as a &Database.
impl Deref for MongoDatabase {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
