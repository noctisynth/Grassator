mod models;
mod utils;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_file_size])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
