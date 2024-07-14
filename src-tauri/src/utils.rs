use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
    sync::Arc,
};

use reqwest::header::CONTENT_LENGTH;
use reqwest::header::RANGE;
use tauri::async_runtime::{spawn, Mutex};
use tauri::Manager;

pub(crate) async fn get_file_size(url: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.head(url).send().await?;
    let size = response
        .headers()
        .get(CONTENT_LENGTH)
        .ok_or("No content-length header")?
        .to_str()?
        .parse::<u64>()?;
    Ok(size)
}

pub(crate) async fn download_file(
    url: &str,
    output_path: &str,
    config: &crate::state::InnerConfig,
) -> Result<bool, Box<dyn std::error::Error>> {
    let output = Arc::new(Mutex::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(output_path)
            .unwrap(),
    ));

    let num_threads = config.num_threads.unwrap_or(12);

    let file_size = get_file_size(url).await?;
    let mut handles = Vec::new();
    let chunk_size = file_size / num_threads as u64;

    for i in 0..num_threads {
        let start = i as u64 * chunk_size;
        let end = if i == num_threads - 1 {
            file_size - 1
        } else {
            (i as u64 + 1) * chunk_size - 1
        };

        let url = url.to_string();
        let output = Arc::clone(&output);
        let handle = spawn(async move {
            download_chunk(&url, start, end, &output).await.unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
    Ok(true)
}

pub(crate) async fn download_chunk(
    url: &str,
    start: u64,
    end: u64,
    output: &Arc<Mutex<std::fs::File>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let range_header = format!("bytes={}-{}", start, end);
    let response = client.get(url).header(RANGE, range_header).send().await?;

    let mut file = output.lock().await;
    file.seek(SeekFrom::Start(start))?;
    let content = response.bytes().await?;
    file.write_all(&content)?;

    Ok(())
}

pub(crate) fn get_config_path_buf<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
) -> Result<std::path::PathBuf, crate::models::ErrorPayload> {
    match app.path().app_config_dir() {
        Ok(mut dir) => {
            if let Err(e) = std::fs::create_dir_all(&dir) {
                return Err(crate::models::ErrorPayload {
                    message: format!("Failed to create app config directory: {}", e),
                });
            };
            dir.push("config.toml");
            Ok(dir)
        }
        Err(e) => Err(crate::models::ErrorPayload {
            message: format!("Failed to get app config directory: {}", e),
        }),
    }
}
