use rbatis::crud::CRUD;
use tide::Request;
use tide::{Response, StatusCode};

use crate::paste::Paste;
use crate::state::State;
use crate::templates::not_found::NotFoundTemplate;

pub async fn download_paste(req: Request<State>) -> tide::Result<Response> {
    let id = req.param("id").unwrap();

    let state = req.state();
    let pool = state.pool.lock().await;

    let mut response: Response;

    let paste: Option<Paste> = pool.fetch_by_column("id", &id.to_string()).await.unwrap();

    match paste {
        Some(paste) => {
            let content = paste.content.unwrap();
            response = Response::builder(200)
                .body(content)
                .header("Content-Transfer-Encoding", "Binary")
                .header("Content-disposition", format!("attachment; filename=\"{}\"", paste.filename.unwrap()))
                .build();
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
