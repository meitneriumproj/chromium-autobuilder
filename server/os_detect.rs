#[derive(Debug)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Unknown,
}

pub fn detect_os(user_agent: &str) -> Platform {

    let ua = user_agent.to_lowercase();

    if ua.contains("windows") {
        Platform::Windows
    } else if ua.contains("mac os") || ua.contains("macintosh") {
        Platform::MacOS
    } else if ua.contains("linux") {
        Platform::Linux
    } else {
        Platform::Unknown
    }
}