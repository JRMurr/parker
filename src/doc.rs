use rocket_contrib::{Template, Value};

pub struct Document {
    pub doc_id: String,
}

impl Document {
    fn get_context(&self) -> Value {
        json!({
            "title": "Poopus",
            "name": "Moopus",
        })
    }

    pub fn render(&self) -> Template {
        let context = self.get_context();
        Template::render("index", context)
    }
}
