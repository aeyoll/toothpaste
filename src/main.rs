#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rbatis;

use askama::Template;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use tide::Request;
use tide::{Response, StatusCode};

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

const THEME: &str = "base16-eighties.dark";

#[derive(Template)]
#[template(path = "get_paste.html")]
struct GetPasteTemplate<'a> {
    filename: &'a str,
    content: &'a str,
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate<'a> {
    message: &'a str,
}

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}

#[crud_table]
#[derive(Clone, Debug)]
struct Paste {
    pub id: Option<u32>,
    pub filename: Option<String>,
    pub content: Option<String>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv::dotenv().ok();
    let mut app = tide::new();

    // set up database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    RB.link(&database_url).await.unwrap();

    app.at("/").get(index);
    app.at("/new").post(new);
    app.at("/paste/:id").get(get_paste);

    let addr = "127.0.0.1:8080";
    println!("http server listen on http://{}", addr);

    app.listen(addr).await?;
    Ok(())
}

pub async fn index(req: Request<()>) -> tide::Result<String> {
    let v = RB.fetch_list::<Paste>().await;
    Ok(serde_json::json!(v).to_string())
}

pub async fn new(req: Request<()>) -> tide::Result<String> {
    Ok("".to_string())
}

pub async fn get_paste(req: Request<()>) -> tide::Result<Response> {
    let id = req.param("id").unwrap();
    let paste: Option<Paste> = RB.fetch_by_column("id", &id.to_string()).await.unwrap();

    let mut response: Response;

    match paste {
        Some(paste) => {
            let s = &paste.content.unwrap();
            let ss = SyntaxSet::load_defaults_newlines();
            let syntax = ss.find_syntax_by_extension("sql").unwrap();
            let ts = ThemeSet::load_defaults();

            let html_content = highlighted_html_for_string(
                s,
                &ss,
                syntax,
                &ts.themes[THEME]
            );
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
