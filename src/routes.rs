use std::path::PathBuf;

use mongodb::{self, db::ThreadedDatabase};
use rocket::response::status;
use rocket_contrib::{Json, Template};

use database::MongoDatabase;
use doc::WebDocument;

const COLLECTION_NAME: &str = "documents";

fn render_doc(
    db: MongoDatabase,
    doc_route: &str,
) -> mongodb::Result<Option<Template>> {
    let coll = &db.collection(COLLECTION_NAME);
    let doc_opt = WebDocument::find(&coll, doc_route)?;
    Ok(doc_opt.map(|doc| doc.render())) // If the doc is populated, render it
}

#[get("/")]
pub fn get_index(db: MongoDatabase) -> mongodb::Result<Option<Template>> {
    render_doc(db, "index")
}

#[get("/<doc_route..>", rank = 20)]
pub fn get_doc(
    db: MongoDatabase,
    doc_route: PathBuf,
) -> mongodb::Result<Option<Template>> {
    render_doc(db, &doc_route.to_string_lossy())
}

#[post("/", format = "application/json", data = "<doc>")]
pub fn post(
    db: MongoDatabase,
    doc: Json<WebDocument>,
) -> mongodb::Result<status::Created<Json<WebDocument>>> {
    doc.insert(&db.collection("documents"))
        .map(|_| status::Created(doc.route.clone(), Some(doc)))
}
