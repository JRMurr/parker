use std::collections::HashMap;

use bson::Bson;
use rocket::{
    request::{self, FromRequest},
    Outcome, Request, State,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct RenderContext(HashMap<String, Bson>);

impl RenderContext {
    pub fn merge(&mut self, other_context: RenderContext) {
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
