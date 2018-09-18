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

use rocket_contrib::Template;

mod doc;
mod db;
mod routes;

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .manage(db::MongoClient::connect())
        .mount("/", routes![routes::get, routes::get_json])
        .launch();
}
