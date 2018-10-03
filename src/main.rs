#![feature(decl_macro, never_type, plugin, proc_macro_non_items)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate bson;
extern crate config;
extern crate mongodb;
extern crate pulldown_cmark;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate structopt;

mod database;
mod doc;
mod opt_config;
mod render;
mod routes;

use rocket::routes;
use rocket_contrib::{static_files::StaticFiles, Template};
use structopt::StructOpt;

use opt_config::{Config, Opt};

fn main() {
    let opt = Opt::from_args(); // Load command line args
    let cfg = Config::load(&opt.config_file); // Load cfg from file

    // Start the server
    rocket::ignite()
        .attach(Template::custom(render::init_template_engines))
        .manage(
            database::MongoDatabase::connect(
                &cfg.database_uri,
                &cfg.database_name,
            ).unwrap(),
        ).mount(
            "/",
            routes![routes::get_index, routes::get_doc, routes::post],
        ).mount("/static", StaticFiles::from(cfg.static_dir))
        .launch();
}
