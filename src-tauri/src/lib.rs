mod models;
mod state;
mod utils;

#[tauri::command]
fn get_config(config: tauri::State<state::Config>) -> Result<state::Config, ()> {
    Ok(config.inner().clone())
}

#[tauri::command]
async fn get_file_size(url: &str) -> Result<models::FileSize, ()> {
    match utils::get_file_size(url).await {
        Ok(size) => Ok(models::FileSize { size, error: None }),
        Err(e) => Ok(models::FileSize {
            size: 0,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
async fn download_file(
    url: &str,
    output: &str,
    config: tauri::State<'_, state::Config>,
) -> Result<models::DownloadResult, ()> {
    match utils::download_file(url, output, &config.inner()).await {
        Ok(_) => Ok(models::DownloadResult {
            file_path: output.to_string(),
            error: None,
        }),
        Err(e) => Ok(models::DownloadResult {
            file_path: String::new(),
            error: Some(e.to_string()),
        }),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(state::Config::default())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_config,
            get_file_size,
            download_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
