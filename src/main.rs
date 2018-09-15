#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::env;

use mongodb::{db::ThreadedDatabase, Client, ThreadedClient};
use rocket_contrib::Template;

mod doc;
mod routes;

fn main() {
    let uri = &env::var("DATABASE_URL").expect("DATABASE_URL not specified");
    let client = Client::with_uri(uri).unwrap();
    let coll = client.db("test").collection("documents");

    rocket::ignite()
        .attach(Template::fairing())
        .manage(coll)
        .mount("/", routes![routes::get, routes::get_json])
        .launch();
}
