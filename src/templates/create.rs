use askama::Template;

#[derive(Template)]
#[template(path = "create.html")]
pub struct CreateTemplate {}
