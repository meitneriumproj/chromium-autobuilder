use crate::os_detect::Platform;

pub fn download_path(platform: &Platform) -> String {

    match platform {

        Platform::Windows =>
            "/artifacts/latest/chromium-win.exe".to_string(),

        Platform::MacOS =>
            "/artifacts/latest/chromium-macos.dmg".to_string(),

        Platform::Linux =>
            "/artifacts/latest/chromium-linux.tar.xz".to_string(),

        Platform::Unknown =>
            "/artifacts/latest/".to_string(),
    }
}