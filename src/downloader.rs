use indicatif::{ProgressBar, ProgressStyle, ProgressState};
use tokio::io::AsyncWriteExt;
use std::error::Error;
use std::path::Path;
use reqwest::Client;
use std::time::Duration;
use std::fmt::Write;

pub async fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut response = client.get(url).send().await?;
    let content_length = response.content_length().unwrap_or(0);

    let pb = ProgressBar::new(content_length);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
            .progress_chars("#>-"));
    pb.set_message("Downloading...");
    pb.enable_steady_tick(Duration::from_millis(100));

    let mut file = tokio::fs::File::create(path).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write(&chunk).await?;
        pb.inc(chunk.len() as u64);
    }

    pb.finish_and_clear();

    Ok(())
}
