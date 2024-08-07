use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use std::error::Error;
use std::path::Path;
use reqwest::Client;

pub async fn download_file(url: &str, path: &Path, progress_tx: mpsc::Sender<u64>, length_tx: mpsc::Sender<Option<u64>>) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: HTTP {}", response.status()).into());
    }

    let content_length = response.content_length();
    length_tx.send(content_length).await?;

    let mut downloaded: u64 = 0;

    let mut file = tokio::fs::File::create(path).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        progress_tx.send(downloaded).await?;
    }

    Ok(())
}
