use std::collections::HashMap as Map;

use bson::{self, Bson};
use mongodb::{
    self,
    coll::{results::InsertOneResult, Collection},
};
use rocket_contrib::Template;

#[derive(Serialize, Deserialize)]
pub struct WebDocument {
    slug: String,
    template: String,
    #[serde(flatten)]
    context: Map<String, Bson>,
}

impl WebDocument {
    pub fn find(coll: &Collection, slug: &str) -> mongodb::Result<Option<WebDocument>> {
        // Get the value from the DB
        let val_opt = coll.find_one(Some(doc! { "slug" => slug }), None)?;
        // Parse to a WebDocument
        Ok(match val_opt {
            Some(val) => bson::from_bson(Bson::Document(val))?,
            None => None,
        })
    }

    pub fn insert(&self, coll: &Collection) -> mongodb::Result<InsertOneResult> {
        match bson::to_bson(self).unwrap() {
            Bson::Document(doc) => coll.insert_one(doc, None),
            _ => panic!("No buen"), // TODO
        }
    }

    pub fn render(self, render_context: &Map<String, Bson>) -> Template {
        // Merge the global render context with the context for this document
        let mut context = render_context.clone();
        context.extend(self.context);
        Template::render(self.template, context)
    }
}
