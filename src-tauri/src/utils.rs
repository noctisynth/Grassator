use std::{
    fs::OpenOptions,
    future::Future,
    io::{Seek, SeekFrom, Write},
    pin::Pin,
    sync::Arc,
    task::Poll,
};

use parking_lot::Mutex;
use reqwest::header::CONTENT_LENGTH;
use reqwest::header::RANGE;
use tauri::Manager;
use tokio::sync::{Semaphore, TryAcquireError};

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

    let chunk_size = 10240;
    let mut delivered = 0;

    let semaphore = Arc::new(Semaphore::new(num_threads as usize));

    while delivered < file_size {
        let start = delivered;
        let end = file_size + chunk_size.min(file_size - delivered);
        delivered += end - start;

        let future = DownloadChunk {
            url: url.to_string(),
            start,
            end,
            output: output.clone(),
            client: reqwest::Client::new(),
            semaphore: semaphore.clone(),
            response: None,
            content: None,
        };
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

    let mut file = output.lock();
    file.seek(SeekFrom::Start(start))?;
    let content = response.bytes().await?;
    file.write_all(&content)?;

    Ok(())
}

pub(crate) struct DownloadChunk {
    url: String,
    start: u64,
    end: u64,
    output: Arc<Mutex<std::fs::File>>,
    client: reqwest::Client,
    semaphore: Arc<Semaphore>,
    response: Option<reqwest::Response>,
    content: Option<reqwest::Body>,
}

impl Future for DownloadChunk {
    type Output = Result<bool, Box<dyn std::error::Error>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let permit = match self.semaphore.try_acquire() {
            Ok(permit) => permit,
            Err(TryAcquireError::NoPermits) => return Poll::Pending,
            Err(TryAcquireError::Closed) => {
                return Poll::Ready(Err(TryAcquireError::Closed.into()))
            }
        };

        let range_header = format!("bytes={}-{}", self.start, self.end);

        // let response = if self.response.is_none() {
        //     let response = Pin::new(
        //         &mut self
        //             .client
        //             .get(&self.url)
        //             .header(RANGE, range_header)
        //             .send(),
        //     )
        //     .poll(cx)?;
        // } else {
        //     self.response.take().unwrap()
        // };

        let mut file = self.output.lock();
        Poll::Pending
    }
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
