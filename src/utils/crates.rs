#![allow(unused)]
use anyhow::Result;

fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("ghactions")
        .build()
        .unwrap()
}

pub async fn get_latest(name: String) -> Result<CrateVersion> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("Empty crate name"));
    }

    let url = format!("https://crates.io/api/v1/crates/{}/versions", name);
    log::debug!("URL: {}", url);

    let client = client();
    let resp = client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await?;

    if resp.status() != 200 {
        log::error!("Error :: {:?}", resp);
        return Err(anyhow::anyhow!("Failed to get crate version"));
    }

    let json = resp.json::<CrateVersions>().await?;

    for version in json.versions.iter() {
        log::debug!("Crate Version: {:?}", version);

        if version.crate_name != name {
            return Err(anyhow::anyhow!("Crate name mismatch"));
        }
        if version.yanked {
            log::debug!("Version is yanked");
            continue;
        }

        return Ok(version.clone());
    }
    Err(anyhow::anyhow!("No versions found"))
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CrateVersions {
    pub versions: Vec<CrateVersion>,
    pub meta: CrateMeta,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CrateVersion {
    pub id: u32,
    #[serde(rename = "crate")]
    pub crate_name: String,

    pub num: String,

    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,

    pub downloads: u32,

    pub yanked: bool,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CrateMeta {
    pub total: u32,
    pub next_page: Option<u32>,
}
