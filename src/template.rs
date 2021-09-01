use askama::Template;

#[derive(Template)]
#[template(path = "get_paste.html")]
pub struct GetPasteTemplate<'a> {
    pub filename: &'a str,
    pub content: &'a str,
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundTemplate<'a> {
    pub message: &'a str,
}
