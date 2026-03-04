use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct BuildState {

    pub commit: Option<String>,
    pub status: String,
    pub logs: Vec<String>,
}

impl BuildState {

    pub fn new() -> Arc<Mutex<Self>> {

        Arc::new(Mutex::new(Self {

            commit: None,
            status: "idle".into(),
            logs: Vec::new(),
        }))
    }
}