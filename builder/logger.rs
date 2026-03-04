use std::fs::OpenOptions;
use std::io::Write;

pub fn log(line: &str) {

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("artifacts/latest/build.log")
        .unwrap();

    writeln!(file, "{}", line).unwrap();
}