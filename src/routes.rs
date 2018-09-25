use mongodb::db::ThreadedDatabase;
use rocket_contrib::{Json, Template};

use database::MongoDatabase;
use doc::WebDocument;
use render::RenderContext;

fn render_doc(db: MongoDatabase, render_context: RenderContext, doc_id: &str) -> Option<Template> {
    let coll = &db.collection("documents");
    let doc_opt = WebDocument::find(&coll, doc_id).unwrap();
    doc_opt.map(|doc| doc.render(render_context))
}

#[get("/")]
pub fn get_index(db: MongoDatabase, render_context: RenderContext) -> Option<Template> {
    render_doc(db, render_context, "index")
}

#[get("/<doc_id>")]
pub fn get(db: MongoDatabase, render_context: RenderContext, doc_id: String) -> Option<Template> {
    render_doc(db, render_context, &doc_id)
}

#[post("/", format = "application/json", data = "<doc>")]
pub fn post(db: MongoDatabase, doc: Json<WebDocument>) {
    doc.insert(&db.collection("documents"));
}
