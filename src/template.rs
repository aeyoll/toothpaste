use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "get_paste.html")]
pub struct GetPasteTemplate<'a> {
    pub id: &'a u32,
    pub filename: &'a str,
    pub content: &'a str,
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundTemplate<'a> {
    pub message: &'a str,
}
