use std::collections::HashMap;

use pulldown_cmark::{html, Parser};
use rocket_contrib::{
    tera::{self, try_get_value, Value},
    Engines,
};

fn markdown_filter(
    value: Value,
    _: HashMap<String, Value>,
) -> Result<Value, tera::Error> {
    let s = try_get_value!("markdown", "value", String, value);

    // Convert markdown to HTML
    let parser = Parser::new(s.as_str());
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    tera::to_value(html_buf)
        .map_err(|err| tera::Error::from_kind(tera::ErrorKind::Json(err)))
}

pub fn init_template_engines(engines: &mut Engines) {
    engines.tera.register_filter("markdown", markdown_filter);
}
