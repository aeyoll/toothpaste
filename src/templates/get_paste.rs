use askama::Template;

#[derive(Template)]
#[template(path = "get_paste.html")]
pub struct GetPasteTemplate<'a> {
    pub id: &'a str,
    pub filename: &'a str,
    pub content: &'a str,
}
