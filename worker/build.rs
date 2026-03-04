

use std::{
    process::{Command, Stdio},
    io::{BufRead, BufReader},
    fs,
    sync::{Arc, Mutex},
};

use crate::state::BuildState;

fn log(state: &Arc<Mutex<BuildState>>, line: &str) {

    println!("{}", line);

    let mut s = state.lock().unwrap();

    s.logs.push(line.to_string());
}

fn run(cmd: &mut Command, state: &Arc<Mutex<BuildState>>) -> bool {

    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    for line in reader.lines() {

        let line = line.unwrap();

        log(state, &line);
    }

    child.wait().unwrap().success()
}

fn cpu_count() -> String {

    match std::thread::available_parallelism() {

        Ok(n) => n.get().to_string(),
        Err(_) => "8".into(),
    }
}

fn setup_ccache(state: &Arc<Mutex<BuildState>>) {

    log(state, "Configuring ccache");

    run(
        Command::new("ccache")
            .args(["--max-size", "50G"]),
        state,
    );

    std::env::set_var("CCACHE_DIR", "./ccache");

    std::env::set_var("CC", "ccache clang");
    std::env::set_var("CXX", "ccache clang++");
}

pub fn run_build(commit: String, state: Arc<Mutex<BuildState>>) {

    {
        let mut s = state.lock().unwrap();

        s.status = "building".into();
        s.commit = Some(commit.clone());
        s.logs.clear();
    }

    log(&state, "Starting Chromium build");

    setup_ccache(&state);

    if !std::path::Path::new("chromium").exists() {

        log(&state, "Cloning Chromium source");

        run(
            Command::new("git")
                .args([
                    "clone",
                    "https://chromium.googlesource.com/chromium/src",
                    "chromium"
                ]),
            &state,
        );
    }

    log(&state, "Fetching updates");

    run(
        Command::new("git")
            .current_dir("chromium")
            .args(["fetch"]),
        &state,
    );

    log(&state, &format!("Checking out commit {}", commit));

    run(
        Command::new("git")
            .current_dir("chromium")
            .args(["checkout", &commit]),
        &state,
    );

    log(&state, "Generating GN build");

    run(
        Command::new("gn")
            .current_dir("chromium")
            .args([
                "gen",
                "out/Release",
                "--args=is_component_build=false is_debug=false use_ccache=true"
            ]),
        &state,
    );

    let threads = cpu_count();

    log(
        &state,
        &format!("Running Ninja with {} threads", threads)
    );

    run(
        Command::new("ninja")
            .current_dir("chromium")
            .args([
                "-C",
                "out/Release",
                "chrome",
                "-j",
                &threads
            ]),
        &state,
    );

    fs::create_dir_all("artifacts").unwrap();

    log(&state, "Packaging build");

    run(
        Command::new("tar")
            .args([
                "-cJf",
                "artifacts/chromium.tar.xz",
                "-C",
                "chromium/out/Release",
                "chrome"
            ]),
        &state,
    );

    let mut s = state.lock().unwrap();

    s.status = "done".into();

    drop(s);

    log(&state, "Build finished successfully");
}