use std::collections::HashMap;

use bson::Bson;
use pulldown_cmark::{html, Parser};
use rocket::{
    request::{self, FromRequest},
    Outcome, Request, State,
};
use rocket_contrib::{
    tera::{self, try_get_value, Value},
    Engines,
};
// use tera::{self, Value};

#[derive(Clone, Serialize, Deserialize)]
pub struct RenderContext(HashMap<String, Bson>);

impl RenderContext {
    pub fn extend(&mut self, other_context: RenderContext) {
        self.0.extend(other_context.0);
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for RenderContext {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<RenderContext, Self::Error> {
        let context = request.guard::<State<RenderContext>>()?;
        // Each request needs its own clone for the render. The global
        // context should be small so this clone won't be too expensive.
        Outcome::Success(context.clone())
    }
}

fn markdown_filter(value: Value, _: HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = try_get_value!("markdown", "value", String, value);
    let parser = Parser::new(s.as_str());
    let mut html_buf = String::new();
    tera::to_value(html::push_html(&mut html_buf, parser))
        .map_err(|err| tera::Error::from_kind(tera::ErrorKind::Json(err)))
}

pub fn init_template_engines(engines: &mut Engines) {
    engines.tera.register_filter("markdown", markdown_filter);
}
