use std::collections::HashMap;

use bson::{self, Bson};
use mongodb::{
    self,
    coll::{results::InsertOneResult, Collection},
};
use rocket_contrib::Template;

#[derive(Serialize, Deserialize)]
pub struct WebDocument {
    pub route: String,
    pub template: String,
    #[serde(flatten)]
    extra: HashMap<String, Bson>,
}

impl WebDocument {
    pub fn find(
        coll: &Collection,
        route: &str,
    ) -> mongodb::Result<Option<WebDocument>> {
        // Get the value from the DB
        let val_opt = coll.find_one(Some(doc! { "route" => route }), None)?;
        // Parse to a WebDocument
        Ok(match val_opt {
            Some(val) => bson::from_bson(Bson::Document(val))?,
            None => None,
        })
    }

    pub fn insert(
        &self,
        coll: &Collection,
    ) -> mongodb::Result<InsertOneResult> {
        match bson::to_bson(self)? {
            Bson::Document(doc) => coll.insert_one(doc, None),
            // I don't even know what could cause this case
            _ => panic!("WebDocument did not serialize to Document!"),
        }
    }

    pub fn render(self) -> Template {
        Template::render(self.template.clone(), self)
    }
}
