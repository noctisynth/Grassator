use std::future::Future;
use std::pin::Pin;

pub(crate) type ResponseFuture =
    Pin<Box<dyn Future<Output = Result<reqwest::Response, reqwest::Error>> + Send>>;
pub(crate) type ContentFuture =
    Pin<Box<dyn Future<Output = Result<bytes::Bytes, reqwest::Error>> + Send>>;
