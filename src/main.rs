#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate bson;
extern crate mongodb;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_json;

use rocket_contrib::Template;

mod doc;
mod routes;

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![routes::get])
        .launch();
}
