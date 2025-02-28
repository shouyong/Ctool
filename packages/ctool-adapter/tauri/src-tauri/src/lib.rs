use reqwest::blocking::Client;
use std::path::PathBuf;
use std::time::Duration;
use tauri::WebviewWindow;

fn check_file_exists_in_home_dir(file_path: &str) -> bool {
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => return false,
    };

    let file_path = PathBuf::from(&home_dir).join(file_path);

    if let Err(_) = file_path.metadata() {
        return false;
    }

    file_path.exists()
}

fn is_url_accessible(url: &str) -> bool {
    let client = Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap();
    match client.head(url).send() {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

#[tauri::command]
fn toggle_dev_tools(window: WebviewWindow) {
    if !window.is_devtools_open() {
        window.open_devtools();
    } else {
        window.close_devtools();
    }
}

#[tauri::command]
fn ctool_is_use_offline() -> bool {
    // 强制使用本地离线版本 标示文件 ~/.ctool.tauri_use_local_file.lock
    if check_file_exists_in_home_dir(".ctool.tauri_use_local_file.lock") {
        return true;
    }
    // 验证`ctool.dev`是否可以访问
    !is_url_accessible("https:/ctool.dev/_status.html")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            toggle_dev_tools,
            ctool_is_use_offline
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
