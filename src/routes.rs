use mongodb::coll::Collection;
use rocket::State;
use rocket_contrib::{Json, Template};

use doc::Document;

#[get("/<doc_id>")]
pub fn get(coll: State<Collection>, doc_id: String) -> Option<Template> {
    let doc = Document::find(&coll, doc_id).unwrap();
    doc.map(|d| d.render())
}

#[get("/<doc_id>/json")]
pub fn get_json(coll: State<Collection>, doc_id: String) -> Option<Json<Document>> {
    let doc = Document::find(&coll, doc_id).unwrap();
    doc.map(Json)
}
