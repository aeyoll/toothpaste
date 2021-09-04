use askama::Template;

use crate::paste::Paste;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub pastes: Vec<Paste>,
}
