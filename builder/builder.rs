/*
Copyright 2026 the Persimmon Authors.
See legal/license info in the LICENSE file.
*/

use std::{
    fs,
    time::Duration,
};

use serde::{Deserialize, Serialize};
use reqwest::Client;
use tokio::time::sleep;
use chrono::Utc;

use tokio::sync::broadcast;

pub static mut BUILD_CHANNEL: Option<broadcast::Sender<String>> = None;

const COMMIT_API: &str =
"https://chromium.googlesource.com/chromium/src/+log/main?format=JSON";

const POLL_INTERVAL: u64 = 900;

#[derive(Clone)]
struct Worker {

    name: &'static str,
    url: &'static str,
    platform: Platform,
}

#[derive(Clone)]
enum Platform {

    Linux,
    Windows,
    MacOS,
}

#[derive(Deserialize)]
struct CommitResponse {

    log: Vec<Commit>,
}

#[derive(Deserialize)]
struct Commit {

    commit: String,
}

#[derive(Serialize)]
struct BuildInfo {

    version: String,
    commit: String,
    status: String,
    build_time: String,

    linux_size: u64,
    windows_size: u64,
    macos_size: u64,
}

fn log(msg: &str) {

    println!("{}", msg);

    unsafe {

        if let Some(tx) = &BUILD_CHANNEL {

            let _ = tx.send(msg.to_string());
        }
    }
}

fn workers() -> Vec<Worker> {

    vec![
        Worker {
            name: "linux-builder-1",
            url: "http://linux-builder-1:9000",
            platform: Platform::Linux,
        },
        Worker {
            name: "linux-builder-2",
            url: "http://linux-builder-2:9000",
            platform: Platform::Linux,
        },
        Worker {
            name: "windows-builder",
            url: "http://windows-builder:9000",
            platform: Platform::Windows,
        },
        Worker {
            name: "mac-builder",
            url: "http://mac-builder:9000",
            platform: Platform::MacOS,
        },
    ]
}

async fn latest_commit(client: &Client) -> Option<String> {

    let res = client.get(COMMIT_API).send().await.ok()?.text().await.ok()?;

    let cleaned = res.trim_start_matches(")]}'");

    let parsed: CommitResponse = serde_json::from_str(cleaned).ok()?;

    parsed.log.first().map(|c| c.commit.clone())
}

async fn trigger_build(client: &Client, worker: &Worker, commit: &str) {

    log(&format!("Dispatching {} to {}", commit, worker.name));

    let _ = client
        .post(format!("{}/build", worker.url))
        .json(&serde_json::json!({
            "commit": commit
        }))
        .send()
        .await;
}

async fn worker_done(client: &Client, worker: &Worker) -> bool {

    if let Ok(res) =
        client.get(format!("{}/status", worker.url)).send().await
    {
        if let Ok(text) = res.text().await {

            return text.contains("done");
        }
    }

    false
}

async fn download_artifact(client: &Client, worker: &Worker) -> u64 {

    log(&format!("Fetching artifact from {}", worker.name));

    let filename = match worker.platform {

        Platform::Linux => "chromium-linux.tar.xz",
        Platform::Windows => "chromium-win.exe",
        Platform::MacOS => "chromium-macos.dmg",
    };

    let url = format!("{}/artifact", worker.url);

    if let Ok(res) = client.get(url).send().await {

        if let Ok(bytes) = res.bytes().await {

            fs::create_dir_all("artifacts/latest").unwrap();

            let path = format!("artifacts/latest/{}", filename);

            fs::write(&path, &bytes).unwrap();

            return bytes.len() as u64;
        }
    }

    0
}

async fn run_pipeline(commit: &str) {

    log("Starting distributed CI pipeline");

    let client = Client::new();

    let workers = workers();

    for worker in &workers {

        trigger_build(&client, worker, commit).await;
    }

    log("Waiting for workers");

    loop {

        let mut finished = 0;

        for worker in &workers {

            if worker_done(&client, worker).await {

                finished += 1;
            }
        }

        if finished == workers.len() {

            break;
        }

        sleep(Duration::from_secs(10)).await;
    }

    log("All workers finished");

    let mut linux_size = 0;
    let mut windows_size = 0;
    let mut macos_size = 0;

    for worker in &workers {

        let size = download_artifact(&client, worker).await;

        match worker.platform {

            Platform::Linux => linux_size = size,
            Platform::Windows => windows_size = size,
            Platform::MacOS => macos_size = size,
        }
    }

    let info = BuildInfo {

        version: "chromium-nightly".into(),
        commit: commit.into(),
        status: "success".into(),
        build_time: Utc::now().to_rfc3339(),

        linux_size,
        windows_size,
        macos_size,
    };

    let json = serde_json::to_string_pretty(&info).unwrap();

    fs::write("artifacts/latest/build.json", json).unwrap();

    log("CI pipeline completed");
}

pub async fn start_master() {

    let client = Client::new();

    let mut last_commit = String::new();

    log("Build master started");

    loop {

        if let Some(commit) = latest_commit(&client).await {

            if commit != last_commit {

                log(&format!("New commit {}", commit));

                run_pipeline(&commit).await;

                last_commit = commit;
            }
        }

        sleep(Duration::from_secs(POLL_INTERVAL)).await;
    }
}