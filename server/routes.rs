use axum::{
    response::{Html, Redirect},
    extract::TypedHeader,
    headers::UserAgent,
};
use std::fs;

use crate::os_detect::detect_os;
use crate::download::download_path;

pub async fn index() -> Html<String> {

    let html = fs::read_to_string("web/index.html")
        .unwrap_or("<h1>Chromium Autobuilder</h1>".to_string());

    Html(html)
}

pub async fn latest_download(
    user_agent: Option<TypedHeader<UserAgent>>,
) -> Redirect {

    let ua = user_agent
        .map(|ua| ua.to_string())
        .unwrap_or_default();

    let os = detect_os(&ua);

    let path = download_path(&os);

    Redirect::temporary(&path)
}