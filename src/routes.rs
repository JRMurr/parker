use std::collections::HashMap as Map;

use mongodb::{db::ThreadedDatabase, ThreadedClient};
use rocket::State;
use rocket_contrib::Template;

use db::MongoClient;
use doc::Document;

fn render_doc(
    client: MongoClient,
    render_context: &Map<String, String>,
    doc_id: &str,
) -> Option<Template> {
    let coll = &client.db("test").collection("documents");
    let doc_opt = Document::find(&coll, doc_id).unwrap();
    doc_opt.map(|doc| doc.render(render_context))
}

#[get("/")]
pub fn get_index(
    client: MongoClient,
    render_context: State<Map<String, String>>,
) -> Option<Template> {
    render_doc(client, &render_context, "index")
}

#[get("/<doc_id>")]
pub fn get(
    client: MongoClient,
    render_context: State<Map<String, String>>,
    doc_id: String,
) -> Option<Template> {
    render_doc(client, &render_context, &doc_id)
}
