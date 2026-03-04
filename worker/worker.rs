use axum::{
    Router,
    routing::{get, post},
    extract::State,
    response::IntoResponse,
    Json,
};

use std::sync::{Arc, Mutex};
use tokio::task;

use crate::{state::BuildState, build};

use tower_http::services::ServeFile;

#[derive(serde::Deserialize)]
pub struct BuildRequest {

    pub commit: String,
}

pub fn router(state: Arc<Mutex<BuildState>>) -> Router {

    Router::new()

        .route("/build", post(start_build))
        .route("/status", get(status))
        .route("/logs", get(logs))
        .route("/artifact", get(artifact))

        .with_state(state)
}

async fn start_build(
    State(state): State<Arc<Mutex<BuildState>>>,
    Json(req): Json<BuildRequest>,
) -> impl IntoResponse {

    let commit = req.commit;

    let state_clone = state.clone();

    task::spawn_blocking(move || {

        build::run_build(commit, state_clone);

    });

    "started"
}

async fn status(
    State(state): State<Arc<Mutex<BuildState>>>
) -> impl IntoResponse {

    let s = state.lock().unwrap();

    s.status.clone()
}

async fn logs(
    State(state): State<Arc<Mutex<BuildState>>>
) -> impl IntoResponse {

    let s = state.lock().unwrap();

    Json(&s.logs)
}

async fn artifact() -> impl IntoResponse {

    ServeFile::new("artifacts/chromium.tar.xz")
}