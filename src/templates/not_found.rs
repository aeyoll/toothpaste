use askama::Template;

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundTemplate<'a> {
    pub message: &'a str,
}
