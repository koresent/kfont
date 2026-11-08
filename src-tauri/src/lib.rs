mod config_manager;
mod font_manager;

use config_manager::AppConfig;
use font_manager::{FontDetails, FontSummary};
use std::path::Path;
use tauri::Manager;

#[tauri::command]
fn get_system_fonts_dirs() -> Vec<String> {
    font_manager::get_default_font_dirs()
}

#[tauri::command]
fn scan_font_folders(paths: Vec<String>) -> Vec<FontSummary> {
    let mut all_fonts = Vec::new();
    for p in paths {
        let path = Path::new(&p);
        if path.exists() {
            let fonts = font_manager::scan_path(path);
            all_fonts.extend(fonts);
        }
    }
    all_fonts.sort_by_cached_key(|a| (a.family.to_lowercase(), a.style.to_lowercase()));
    all_fonts
}

#[tauri::command]
fn get_font_details(path: String, face_index: u32) -> Result<FontDetails, String> {
    font_manager::get_font_details(&path, face_index)
}

#[tauri::command]
fn get_font_data_url(path: String) -> Result<String, String> {
    font_manager::get_font_data_url(&path)
}

#[tauri::command]
fn get_font_bytes(path: String) -> Result<tauri::ipc::Response, String> {
    let bytes = std::fs::read(&path).map_err(|e| format!("Failed to read font file: {}", e))?;
    Ok(tauri::ipc::Response::new(bytes))
}

#[tauri::command]
async fn pick_folder() -> Result<Option<String>, String> {
    let folder = rfd::AsyncFileDialog::new()
        .set_title("Select Font Folder")
        .pick_folder()
        .await;

    Ok(folder.map(|f| f.path().to_string_lossy().to_string()))
}

#[tauri::command]
async fn pick_font_files() -> Result<Vec<String>, String> {
    let files = rfd::AsyncFileDialog::new()
        .set_title("Select Font Files")
        .add_filter("Fonts (*.ttf, *.otf, *.ttc, *.woff)", &["ttf", "otf", "ttc", "otc", "woff"])
        .pick_files()
        .await;

    let res = files
        .unwrap_or_default()
        .into_iter()
        .map(|f| f.path().to_string_lossy().to_string())
        .collect();

    Ok(res)
}

#[tauri::command]
fn get_app_config() -> AppConfig {
    config_manager::load_config()
}

#[tauri::command]
fn save_app_config(config: AppConfig) -> Result<(), String> {
    config_manager::save_config(&config)
}

#[tauri::command]
fn get_config_yaml_file() -> Result<String, String> {
    config_manager::get_config_file_path_str()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .register_asynchronous_uri_scheme_protocol("kfont", |_ctx, request, responder| {
            let path_part = request.uri().path().to_string();
            std::thread::spawn(move || {
                let decoded = percent_encoding::percent_decode_str(&path_part)
                    .decode_utf8_lossy()
                    .to_string();
                
                // On unix systems, path_part has leading '/', e.g. /usr/share/fonts/...
                // On windows, path_part might be /C:/Windows/...
                #[cfg(windows)]
                let file_path = if decoded.starts_with('/') && decoded.len() > 2 && decoded.chars().nth(2) == Some(':') {
                    decoded[1..].to_string()
                } else {
                    decoded
                };

                #[cfg(not(windows))]
                let file_path = decoded;

                let path = std::path::Path::new(&file_path);
                if path.exists() && path.is_file() {
                    if let Ok(bytes) = std::fs::read(path) {
                        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
                        let mime = match ext.as_str() {
                            "otf" => "font/otf",
                            "woff" => "font/woff",
                            "woff2" => "font/woff2",
                            _ => "font/ttf",
                        };

                        let response = tauri::http::Response::builder()
                            .header("Access-Control-Allow-Origin", "*")
                            .header("Content-Type", mime)
                            .header("Cache-Control", "public, max-age=31536000, immutable")
                            .body(bytes)
                            .unwrap();
                        responder.respond(response);
                        return;
                    }
                }

                let response = tauri::http::Response::builder()
                    .status(tauri::http::StatusCode::NOT_FOUND)
                    .header("Access-Control-Allow-Origin", "*")
                    .body(Vec::new())
                    .unwrap();
                responder.respond(response);
            });
        })
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                if let Some(icon) = app.default_window_icon() {
                    let _ = window.set_icon(icon.clone());
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_system_fonts_dirs,
            scan_font_folders,
            get_font_details,
            get_font_data_url,
            get_font_bytes,
            pick_folder,
            pick_font_files,
            get_app_config,
            save_app_config,
            get_config_yaml_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
