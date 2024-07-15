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
    let mut futures = vec![];

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
            content_fut: None.into(),
            response_fut: None.into(),
        };
        futures.push(future);
    }

    for future in futures {
        future.await?;
    }
    Ok(true)
}

pub(crate) struct DownloadChunk {
    url: String,
    start: u64,
    end: u64,
    output: Arc<Mutex<std::fs::File>>,
    client: reqwest::Client,
    semaphore: Arc<Semaphore>,
    response_fut: Mutex<Option<crate::types::ResponseFuture>>,
    content_fut: Mutex<Option<crate::types::ContentFuture>>,
}

impl Future for DownloadChunk {
    type Output = Result<(), Box<dyn std::error::Error>>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let permit = match self.semaphore.try_acquire() {
            Ok(permit) => permit,
            Err(TryAcquireError::NoPermits) => return Poll::Pending,
            Err(TryAcquireError::Closed) => {
                return Poll::Ready(Err(TryAcquireError::Closed.into()))
            }
        };

        let range_header = format!("bytes={}-{}", self.start, self.end);

        let mut response_fut = self.response_fut.lock();
        if response_fut.is_none() {
            response_fut.replace(Box::pin(
                self.client
                    .get(&self.url)
                    .header(RANGE, range_header)
                    .send(),
            ));
        };

        let mut content_fut = self.content_fut.lock();
        if content_fut.is_none() {
            content_fut.replace(match response_fut.as_mut().unwrap().as_mut().poll(cx) {
                Poll::Ready(Ok(response)) => Box::pin(response.bytes()),
                Poll::Ready(Err(_)) => {
                    // TODO: emit error event
                    response_fut.take();
                    return Poll::Pending;
                }
                Poll::Pending => return Poll::Pending,
            });
        }

        let content = match content_fut.as_mut().unwrap().as_mut().poll(cx) {
            Poll::Ready(Ok(content)) => content,
            Poll::Ready(Err(_)) => {
                // TODO: emit error event
                response_fut.take();
                content_fut.take();
                return Poll::Pending;
            }
            Poll::Pending => return Poll::Pending,
        };
        let mut file = self.output.lock();
        file.seek(SeekFrom::Start(self.start))?;
        file.write_all(&content)?;
        drop(permit);
        Poll::Ready(Ok(()))
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
