use async_std::{sync::Arc, sync::Mutex};
use std::{clone::Clone, error::Error, str::FromStr};
use tide::{http::Mime, Response, StatusCode, Request};


#[derive(Clone)]
struct State {
    cell: Arc<Mutex<u8>>,
}


#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let cell = Arc::new(Mutex::new(0u8));
    log::info!("i={}", cell.lock().await);

    log::info!("i={}", cell.lock().await);
    let mut app = tide::with_state(State { cell });

    app.at("/").get(|req: Request<State>| async move {
        log::info!("Serving /");
        serve_template(req.state()).await.map_err(|e| {
            log::error!("While serving: {}", e);
            tide::Error::from_str(StatusCode::InternalServerError, "fail!",)
        })
    });
    app.listen("localhost:3001").await?;
    Ok(())
}

async fn serve_template(state: &State) -> Result<Response, Box<dyn Error>> {
    let mut res = Response::new(StatusCode::Ok);
    res.set_content_type(Mime::from_str("text/html; charset=utf-8").unwrap());
    let mut cell = state.cell.lock().await;
    res.set_body(cell.to_string());
    log::info!("i={}", cell);
    *cell += 1;
    Ok(res)
}
