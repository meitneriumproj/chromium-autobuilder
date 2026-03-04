use axum::response::Html;
use std::fs;

pub async fn dashboard() -> Html<String> {

    let html = fs::read_to_string("web/dashboard.html")
        .unwrap_or("<h1>Dashboard unavailable</h1>".to_string());

    Html(html)
}