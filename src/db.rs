use std::ops::Deref;

use mongodb::{Client, ThreadedClient};
use rocket::{
    request::{self, FromRequest},
    Outcome, Request, State,
};

#[derive(Clone)]
pub struct MongoClient(pub Client);

impl MongoClient {
    pub fn connect(db_uri: &str) -> MongoClient {
        let client = Client::with_uri(db_uri).unwrap();
        MongoClient(client)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for MongoClient {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<MongoClient, Self::Error> {
        // The Client type is actually just an Arc so a clone is cheap
        let client = request.guard::<State<MongoClient>>()?;
        Outcome::Success(client.clone())
    }
}

// For the convenience of using an &MongoClient as a &Client.
impl Deref for MongoClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
