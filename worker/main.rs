mod worker;
mod build;
mod state;

use worker::router;
use state::BuildState;

#[tokio::main]
async fn main() {

    let state = BuildState::new();

    let app = router(state);

    println!("Worker listening on port 9000");

    axum::Server::bind(
        &"0.0.0.0:9000".parse().unwrap()
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}