use std::collections::HashMap as Map;

use bson::{self, Bson};
use mongodb::coll::{Collection,results::InsertOneResult};
use mongodb::Error;
use rocket_contrib::Template;

#[derive(Serialize, Deserialize)]
pub struct WebDocument {
    slug: String,
    template: String,
    #[serde(flatten)]
    context: Map<String, String>,
}

impl WebDocument {
    pub fn find(coll: &Collection, slug: &str) -> Result<Option<WebDocument>, Error> {
        let val_opt = coll.find_one(Some(doc! { "slug" => slug }), None).unwrap();
        let doc = match val_opt {
            Some(val) => bson::from_bson(Bson::Document(val)).unwrap(),
            None => None,
        };
        Ok(doc)
    }

    pub fn insert(&self, coll: &Collection) -> Result<InsertOneResult, Error> {
        match bson::to_bson(self).unwrap() {
            Bson::Document(doc) => coll.insert_one(doc, None),
            _ => panic!("No buen"),
        }
    }

    pub fn render(self, render_context: &Map<String, String>) -> Template {
        let context = render_context.clone().extend(self.context);
        Template::render(self.template, context)
    }
}
