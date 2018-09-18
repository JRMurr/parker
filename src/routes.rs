use mongodb::{ThreadedClient,db::ThreadedDatabase};
use rocket_contrib::{Json, Template};

use db::MongoClient;
use doc::Document;

fn get_doc(client: MongoClient, doc_id: &str) -> Option<Document> {
    let coll = &client.db("test").collection("documents");
    Document::find(&coll, doc_id.to_string()).unwrap()
}

#[get("/<doc_id>")]
pub fn get(client: MongoClient, doc_id: String) -> Option<Template> {
    get_doc(client, &doc_id).map(|d| d.render())
}

#[get("/<doc_id>/json")]
pub fn get_json(client: MongoClient, doc_id: String) -> Option<Json<Document>> {
    get_doc(client, &doc_id).map(Json)
}
