#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate bson;
extern crate config;
extern crate mongodb;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate structopt;

use std::collections::HashMap as Map;

use rocket_contrib::Template;
use structopt::StructOpt;

mod db;
mod doc;
mod routes;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Config file path
    #[structopt(short = "c", long = "config", default_value = "parker.toml",)]
    config_file: String,
}

#[derive(Deserialize)]
struct Settings {
    database_uri: String,
}

#[derive(Deserialize)]
struct Cfg {
    settings: Settings,
    render_context: Map<String, String>,
}

fn main() {
    let opt = Opt::from_args();

    let mut cfg_handler = config::Config::default();
    cfg_handler
        .merge(config::File::with_name(&opt.config_file))
        .unwrap();
    let cfg: Cfg = cfg_handler.try_into().unwrap();

    rocket::ignite()
        .attach(Template::fairing())
        .manage(db::MongoClient::connect(&cfg.settings.database_uri))
        .manage(cfg.render_context)
        .mount("/", routes![routes::get_index, routes::get])
        .launch();
}
