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
mod render;
mod routes;

use rocket::routes;
use rocket_contrib::{static_files::StaticFiles, Template};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Parker")]
struct Opt {
    /// Config file path
    #[structopt(short = "c", long = "config", default_value = "parker.toml")]
    config_file: String,
}

#[derive(Deserialize)]
struct Cfg {
    database_uri: String,
    database_name: String,
    static_dir: String,
}

fn main() {
    // Load command line args
    let opt = Opt::from_args();

    // Load cfg
    let mut cfg_handler = config::Config::default();
    cfg_handler
        .merge(config::File::with_name(&opt.config_file))
        .unwrap();
    let cfg: Cfg = cfg_handler.try_into().unwrap();

    rocket::ignite()
        .attach(Template::custom(render::init_template_engines))
        .manage(
            database::MongoDatabase::connect(
                &cfg.database_uri,
                &cfg.database_name,
            )
            .unwrap(),
        )
        .mount(
            "/",
            routes![routes::get_index, routes::get_doc, routes::post],
        )
        .mount("/static", StaticFiles::from(cfg.static_dir))
        .launch();
}
