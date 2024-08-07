use reqwest::get;
use serde::Deserialize;
use serde_json;

const FABRIC_MANIFEST_URL: &str = "https://meta.fabricmc.net/v2/versions";

#[derive(Deserialize)]
struct FabricManifest {
    game: Vec<MinecraftVersion>,
    loader: Vec<LoaderVersion>,
    installer: Vec<InstallerVersion>,
}

#[derive(Deserialize)]
struct MinecraftVersion {
    version: String,
    stable: bool,
}

#[derive(Deserialize)]
struct LoaderVersion {
    version: String,
    stable: bool,
}

#[derive(Deserialize)]
struct InstallerVersion {
    version: String,
    stable: bool,
}

async fn get_fabric_manifest() -> Result<FabricManifest, Box<dyn std::error::Error>> {
    let response = get(FABRIC_MANIFEST_URL).await?.text().await?;
    let manifest: FabricManifest = serde_json::from_str(&response)?;
    Ok(manifest)
}

async fn get_latest_minecraft_version() -> Result<String, Box<dyn std::error::Error>> {
    let manifest = get_fabric_manifest().await?;
    let latest_minecraft_version = manifest.game.iter().find(|v| v.stable).unwrap();
    Ok(latest_minecraft_version.version.clone())
}

async fn get_latest_loader_version(unstable: bool) -> Result<String, Box<dyn std::error::Error>> {
    let manifest = get_fabric_manifest().await?;
    let latest_loader_version = manifest.loader.iter().find(|v| unstable || v.stable).unwrap();
    Ok(latest_loader_version.version.clone())
}

async fn get_latest_installer_version(unstable: bool) -> Result<String, Box<dyn std::error::Error>> {
    let manifest = get_fabric_manifest().await?;
    let latest_installer_version = manifest.installer.iter().find(|v| unstable || v.stable).unwrap();
    Ok(latest_installer_version.version.clone())
}

pub async fn get_download_link(version: Option<String>, loader_version: Option<String>, installer_version: Option<String>, unstable_loader: bool, unstable_installer: bool) -> Result<(String, String), Box<dyn std::error::Error>> {
    let latest_minecraft_version = get_latest_minecraft_version().await?;
    let latest_loader_version = get_latest_loader_version(unstable_loader).await?;
    let latest_installer_version = get_latest_installer_version(unstable_installer).await?;

    let version = match version {
        Some(ref v) if !v.is_empty() => Some(v.clone()),
        _ => Some(latest_minecraft_version.clone())
    };
    let loader_version = match loader_version {
        Some(ref v) if !v.is_empty() => Some(v.clone()),
        _ => Some(latest_loader_version.clone())
    };
    let installer_version = match installer_version {
        Some(ref v) if !v.is_empty() => Some(v.clone()),
        _ => Some(latest_installer_version.clone())
    };

    if version.clone().unwrap() > latest_minecraft_version {
        return Err(format!("Minecraft version {} not found. Latest is {}", version.clone().unwrap(), latest_minecraft_version).into());
    }

    if loader_version.clone().unwrap() > latest_loader_version {
        return Err(format!("Loader version {} not found. Latest is {}", loader_version.clone().unwrap(), latest_loader_version).into());
    }

    if installer_version.clone().unwrap() > latest_installer_version {
        return Err(format!("Installer version {} not found. Latest is {}", installer_version.clone().unwrap(), latest_installer_version).into());
    }

    let url = format!("https://meta.fabricmc.net/v2/versions/loader/{}/{}/{}/server/jar", version.clone().unwrap(), loader_version.clone().unwrap(), installer_version.clone().unwrap());
    let version_info = format!("(Version: {}, Loader: {}, Installer: {})", version.unwrap(), loader_version.unwrap(), installer_version.unwrap());
    Ok((url, version_info))
}
