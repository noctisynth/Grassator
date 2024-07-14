use models::ErrorPayload;

mod models;
mod state;
mod utils;

#[tauri::command]
async fn load_config<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    config: tauri::State<'_, state::Config>,
) -> Result<models::ConfigResult, ErrorPayload> {
    let config_file_path = utils::get_config_path_buf(&app)?;
    let config_data = if !config_file_path.exists() {
        let config_data = toml::to_string_pretty(&state::InnerConfig::default()).unwrap();
        println!("{}", config_data);
        std::fs::write(&config_file_path, &config_data).unwrap();
        config_data
    } else {
        std::fs::read_to_string(&config_file_path).unwrap()
    };
    let inner_config: state::InnerConfig = toml::from_str(&config_data).unwrap();
    *config.inner().inner.write().await = inner_config.clone();
    Ok(models::ConfigResult {
        inner: inner_config,
        file_path: config_file_path.to_string_lossy().to_string(),
    })
}

#[tauri::command]
async fn get_config(config: tauri::State<'_, state::Config>) -> Result<state::InnerConfig, ()> {
    Ok(config.inner().inner.read().await.clone())
}

#[tauri::command]
async fn set_config<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    new_config: state::InnerConfig,
    config: tauri::State<'_, state::Config>,
) -> Result<models::SetConfigResult, models::ErrorPayload> {
    let config_file_path = utils::get_config_path_buf(&app)?;
    let toml = toml::to_string_pretty(&new_config).unwrap();
    if let Err(e) = std::fs::write(&config_file_path, toml) {
        return Err(ErrorPayload {
            message: format!("Failed to update config file: {}", e),
        });
    };
    *config.inner().inner.write().await = new_config;
    Ok(models::SetConfigResult {
        config_path: config_file_path.to_string_lossy().to_string(),
    })
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
    match utils::download_file(url, output, &*config.inner().inner.read().await).await {
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
            load_config,
            get_config,
            set_config,
            get_file_size,
            download_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
