use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
    sync::Arc,
};

use reqwest::header::CONTENT_LENGTH;
use reqwest::header::RANGE;
use tauri::async_runtime::{spawn, Mutex};

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
    config: &crate::state::Config,
) -> Result<bool, Box<dyn std::error::Error>> {
    let output = Arc::new(Mutex::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(output_path)
            .unwrap(),
    ));

    let file_size = get_file_size(url).await?;
    let mut handles = Vec::new();
    let chunk_size = file_size / config.num_threads as u64;

    for i in 0..config.num_threads {
        let start = i as u64 * chunk_size;
        let end = if i == config.num_threads - 1 {
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
