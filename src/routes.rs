use rocket_contrib::Template;

use doc::Document;

#[get("/<doc_id>")]
pub fn get(doc_id: String) -> Template {
    let doc = Document { doc_id };
    doc.render()
}
