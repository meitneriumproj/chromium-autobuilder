

use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct BuildState {

    pub commit: String,
    pub status: String,
}

pub fn save(state: &BuildState) {

    let data = serde_json::to_string_pretty(state).unwrap();

    fs::write("builder/state.json", data).unwrap();
}