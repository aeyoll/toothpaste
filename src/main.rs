#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rbatis;

use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use tide::Request;
use tide::{Response, StatusCode};

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

const THEME: &str = "base16-eighties.dark";

mod cache;
use cache::{HtmlCache, create_cache};

mod template;
use template::{GetPasteTemplate, NotFoundTemplate};

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}

#[crud_table]
#[derive(Clone)]
struct Paste {
    pub id: Option<u32>,
    pub filename: Option<String>,
    pub content: Option<String>,
}

#[derive(Clone)]
pub struct State {
    pub cache: HtmlCache,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Environment
    dotenv::dotenv().ok();

    // Database
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    RB.link(&database_url).await.unwrap();

    // Cache
    let cache = create_cache();

    // State
    let state = State { cache };

    let mut app = tide::with_state(state.clone());

    app.at("/").get(index);
    app.at("/new").post(new);
    app.at("/paste/:id").get(get_paste);

    let addr = "127.0.0.1:8080";
    println!("http server listen on http://{}", addr);

    app.listen(addr).await?;
    Ok(())
}

pub async fn index(req: Request<State>) -> tide::Result<String> {
    let v = RB.fetch_list::<Paste>().await;
    Ok(serde_json::json!(v).to_string())
}

pub async fn new(req: Request<State>) -> tide::Result<String> {
    Ok("".to_string())
}

pub async fn get_paste(req: Request<State>) -> tide::Result<Response> {
    let id = req.param("id").unwrap();

    let state = req.state();
    let mut cache = state.cache.lock().await;
    let cache_key = id.parse::<i64>().unwrap();

    let mut response: Response;

    let paste: Option<Paste> = RB.fetch_by_column("id", &id.to_string()).await.unwrap();

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
