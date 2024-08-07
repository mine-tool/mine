use reqwest::get;
use serde::Deserialize;
use serde_json;

const VANILLA_MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Deserialize)]
struct VanillaManifest {
    latest: VanillaLatest,
    versions: Vec<VanillaVersion>,
}

#[derive(Deserialize)]
struct VanillaLatest {
    release: String,
    snapshot: String,
}

#[derive(Deserialize)]
struct VanillaVersion {
    id: String, // minecraft version
    url: String,
}

#[derive(Deserialize)]
struct VanillaArtifact {
    downloads: VanillaDownloads,
}

#[derive(Deserialize)]
struct VanillaDownloads {
    server: VanillaServerDownloadInfo,
}

#[derive(Deserialize)]
struct VanillaServerDownloadInfo {
    url: String,
}

async fn get_manifest() -> Result<VanillaManifest, Box<dyn std::error::Error>> {
    let response = get(VANILLA_MANIFEST_URL).await?.text().await?;
    let manifest: VanillaManifest = serde_json::from_str(&response)?;
    Ok(manifest)
}

async fn get_artifact(version: String) -> Result<VanillaArtifact, Box<dyn std::error::Error>> {
    let manifest = get_manifest().await?;
    for ver in manifest.versions {
        if ver.id == version {
            let version_artifact = get(&ver.url).await?.json::<VanillaArtifact>().await?;
            return Ok(version_artifact);
        }
    }

    Err("Version not found".into())
}

async fn get_latest_version(snapshot: bool) -> Result<String, Box<dyn std::error::Error>> {
    let manifest = get_manifest().await?;
    Ok(if snapshot { manifest.latest.snapshot } else { manifest.latest.release })
}

pub async fn get_download_link(version: Option<String>, snapshot: bool) -> Result<(String, String), Box<dyn std::error::Error>> {
    let latest_stable_version = get_latest_version(false).await?;
    let latest_snapshot_version = get_latest_version(true).await?;

    let version = match version {
        Some(ref v) if !v.is_empty() => Some(v.clone()),
        _ => Some(if snapshot { latest_snapshot_version.clone() } else { latest_stable_version.clone() })
    };

    let version_id = version.unwrap();
    let manifest = get_manifest().await?;

    for ver in manifest.versions {
        if ver.id == version_id {
            let version_artifact = get_artifact(ver.id.clone()).await?;
            return Ok((
                version_artifact.downloads.server.url,
                format!("({})", ver.id),
            ));
        }
    }

    let latest_minecraft_version = if snapshot { latest_snapshot_version } else { latest_stable_version };
    return Err(format!("Minecraft version {} not found. Latest is {}", version_id, latest_minecraft_version).into());
}
