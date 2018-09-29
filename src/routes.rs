use std::path::PathBuf;

use mongodb::db::ThreadedDatabase;
use rocket_contrib::{Json, Template};

use database::MongoDatabase;
use doc::WebDocument;

fn render_doc(db: MongoDatabase, doc_route: &str) -> Option<Template> {
    let coll = &db.collection("documents");
    let doc_opt = WebDocument::find(&coll, doc_route).unwrap();
    doc_opt.map(|doc| doc.render())
}

#[get("/")]
pub fn get_index(db: MongoDatabase) -> Option<Template> {
    render_doc(db, "index")
}

#[get("/<doc_route..>", rank = 20)]
pub fn get_doc(db: MongoDatabase, doc_route: PathBuf) -> Option<Template> {
    render_doc(db, &doc_route.to_string_lossy())
}

#[post("/", format = "application/json", data = "<doc>")]
pub fn post(db: MongoDatabase, doc: Json<WebDocument>) {
    doc.insert(&db.collection("documents"));
}
