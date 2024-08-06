use reqwest::get;
use serde::Deserialize;

const PAPER_MANIFEST_URL: &str = "https://api.papermc.io/v2/projects/paper";

// https://api.papermc.io/v2/projects/paper
#[derive(Deserialize)]
struct PaperManifest {
    versions: Vec<String>,
}

// https://api.papermc.io/v2/projects/paper/versions/1.16.5
#[derive(Deserialize)]
struct PaperVersion {
    builds: Vec<u32>,
}

// https://api.papermc.io/v2/projects/paper/versions/1.16.5/builds/471
#[derive(Deserialize)]
struct PaperBuild {
    downloads: Downloads,
}

#[derive(Deserialize)]
struct Downloads {
    application: DownloadInfo,
}

#[derive(Deserialize)]
struct DownloadInfo {
    name: String,
}

async fn get_manifest() -> Result<PaperManifest, Box<dyn std::error::Error>> {
    let response = get(PAPER_MANIFEST_URL).await?.text().await?;
    let manifest: PaperManifest = serde_json::from_str(&response)?;
    Ok(manifest)
}

async fn get_latest_version() -> Result<String, Box<dyn std::error::Error>> {
    let manifest = get_manifest().await?;
    // the latest version is the last element
    Ok(manifest.versions.last().unwrap().clone())
}

async fn get_latest_build(version: String) -> Result<u32, Box<dyn std::error::Error>> {
    let response = get(format!("{}/versions/{}", PAPER_MANIFEST_URL, version)).await?.text().await?;
    let paper_version: PaperVersion = serde_json::from_str(&response)?;
    Ok(*paper_version.builds.last().unwrap())
}

async fn get_build(version: String, build: u32) -> Result<PaperBuild, Box<dyn std::error::Error>> {
    let response = get(format!("{}/versions/{}/builds/{}", PAPER_MANIFEST_URL, version, build)).await?.text().await?;
    let paper_build: PaperBuild = serde_json::from_str(&response)?;
    Ok(paper_build)
}

pub async fn get_download_link(version: Option<String>, build: Option<u32>) -> Result<String, Box<dyn std::error::Error>> {
    let latest_version = get_latest_version().await?;
    let latest_build = get_latest_build(latest_version.clone()).await?;

    let version = match version {
        Some(v) if !v.is_empty() => v,
        _ => latest_version.clone(),
    };

    let build = match build {
        Some(b) => b,
        None => latest_build,
    };

    if version > latest_version {
        return Err(format!("Version {} not found. Latest is {}", version, latest_version).into());
    }

    if build > latest_build {
        return Err(format!("Build {} not found. Latest is {}", build, latest_build).into());
    }

    let paper_build = get_build(version.clone(), build).await?;

    Ok(format!("{}/versions/{}/builds/{}/downloads/{}", PAPER_MANIFEST_URL, version, build, paper_build.downloads.application.name))
}
