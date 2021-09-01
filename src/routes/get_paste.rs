use rbatis::crud::CRUD;
use tide::Request;
use tide::{Response, StatusCode};

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

use crate::paste::Paste;
use crate::state::State;
use crate::template::{GetPasteTemplate, NotFoundTemplate};

const THEME: &str = "base16-eighties.dark";

pub async fn get_paste(req: Request<State>) -> tide::Result<Response> {
    let id = req.param("id").unwrap();

    let state = req.state();
    let pool = state.pool.lock().await;
    let mut cache = state.cache.lock().await;
    let cache_key = id.parse::<i64>().unwrap();

    let mut response: Response;

    let paste: Option<Paste> = pool.fetch_by_column("id", &id.to_string()).await.unwrap();

    match paste {
        Some(paste) => {
            let html_content;

            if let Some(response) = cache.get(&cache_key) {
                html_content = response.clone();
            } else {
                let s = &paste.content.unwrap();
                let ss = SyntaxSet::load_defaults_newlines();
                let syntax = ss.find_syntax_by_extension("sql").unwrap();
                let ts = ThemeSet::load_defaults();

                html_content = highlighted_html_for_string(s, &ss, syntax, &ts.themes[THEME]);
                let _ = cache.put_with_weight(cache_key, html_content.clone());
            }

            response = GetPasteTemplate {
                filename: &paste.filename.unwrap(),
                content: &html_content,
            }
            .into();
        }
        None => {
            response = NotFoundTemplate {
                message: "Paste not found",
            }
            .into();
            response.set_status(StatusCode::NotFound);
        }
    }

    Ok(response)
}
