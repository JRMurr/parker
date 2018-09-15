use bson;
use mongodb::coll::Collection;
use mongodb::Error;
use rocket_contrib::Template;

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub doc_id: String,
    pub title: String,
    pub name: String,
}

impl Document {
    pub fn find(coll: &Collection, doc_id: String) -> Result<Option<Document>, Error> {
        let val_opt = coll.find_one(Some(doc! { "_id": doc_id }), None).unwrap();
        let doc = match val_opt {
            Some(val) => bson::from_bson(bson::Bson::Document(val)).unwrap(),
            None => None,
        };
        Ok(doc)
    }

    pub fn render(&self) -> Template {
        Template::render("index", self)
    }
}
