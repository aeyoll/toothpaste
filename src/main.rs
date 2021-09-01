#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rbatis;

use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;
use tide::Request;
use askama::Template;

use tide::{http::mime::HTML, Body, Response};

#[derive(Template)]
#[template(path = "get_paste.html")]
struct GetPasteTemplate<'a> {
    title: &'a str,
    content: &'a str,
}

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}

#[crud_table]
#[derive(Clone, Debug)]
struct Paste {
    pub title: Option<String>,
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
    let res: Response = GetPasteTemplate { title: "world", content: "world" }.into();
    Ok(res)
}