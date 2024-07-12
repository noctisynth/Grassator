pub(crate) async fn get_file_size(url: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.head(url).send().await?;
    let size = response
        .headers()
        .get("content-length")
        .ok_or("No content-length header")?
        .to_str()?
        .parse::<u64>()?;
    Ok(size)
}
