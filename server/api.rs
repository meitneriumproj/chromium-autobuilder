use axum::Json;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct BuildInfo {

    pub version: String,
    pub commit: String,
    pub status: String,
    pub build_time: String,

    pub linux_size: u64,
    pub windows_size: u64,
    pub macos_size: u64,
}

pub async fn latest() -> Json<BuildInfo> {

    let data = fs::read_to_string("artifacts/latest/build.json")
        .unwrap_or_default();

    let info: BuildInfo = serde_json::from_str(&data).unwrap_or(
        BuildInfo {
            version: "unknown".into(),
            commit: "unknown".into(),
            status: "unknown".into(),
            build_time: "unknown".into(),
            linux_size: 0,
            windows_size: 0,
            macos_size: 0,
        }
    );

    Json(info)
}