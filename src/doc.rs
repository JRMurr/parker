use bson::{self, Bson};
use mongodb::{
    self,
    coll::{results::InsertOneResult, Collection},
};
use rocket_contrib::Template;

use render::RenderContext;

#[derive(Serialize, Deserialize)]
pub struct WebDocument {
    slug: String,
    template: String,
    #[serde(flatten)]
    render_context: RenderContext,
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
        match bson::to_bson(self)? {
            Bson::Document(doc) => coll.insert_one(doc, None),
            _ => panic!("No buen"), // TODO
        }
    }

    pub fn render(self, mut render_context: RenderContext) -> Template {
        // Merge document render context into global context
        render_context.extend(self.render_context);
        Template::render(self.template, render_context)
    }
}
