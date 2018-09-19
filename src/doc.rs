use std::collections::HashMap as Map;

use bson::{self, Bson};
use mongodb::coll::Collection;
use mongodb::Error;
use rocket_contrib::Template;

#[derive(Serialize, Deserialize)]
pub struct Document {
    template: String,
    slug: String,
    #[serde(flatten)]
    context: Map<String, String>,
}

impl Document {
    pub fn find(coll: &Collection, slug: &str) -> Result<Option<Document>, Error> {
        let val_opt = coll.find_one(Some(doc! { "slug" => slug }), None).unwrap();
        let doc = match val_opt {
            Some(val) => bson::from_bson(Bson::Document(val)).unwrap(),
            None => None,
        };
        Ok(doc)
    }

    pub fn render(self, render_context: &Map<String, String>) -> Template {
        let context = render_context.clone().extend(self.context);
        Template::render(self.template, context)
    }
}
