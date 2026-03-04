use axum::{
    Router,
    routing::get,
};

use tower_http::services::ServeDir;

mod websocket;

#[tokio::main]
async fn main() {

    let app = Router::new()

        .route("/ws", get(websocket::ws_handler))

        .nest_service("/web", ServeDir::new("./web"))

        .nest_service("/artifacts", ServeDir::new("./artifacts"));

    axum::Server::bind(
        &"0.0.0.0:8080".parse().unwrap()
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}