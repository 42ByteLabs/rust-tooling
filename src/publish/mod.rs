use std::path::PathBuf;

use anyhow::Result;
use ghactions::Actions;

use crate::utils::cargo::Cargo;
use crate::utils::crates;

#[derive(Actions, Debug, Default)]
#[action(
    name = "rust-crate-publish",
    description = "Rust Crate Publish",
    path = "./publish/action.yml",
    image = "./publish/Dockerfile"
)]
pub struct RustCratePublishAction {
    /// Location of the Cargo.toml file
    #[input(
        description = "Main Cargo File for the project",
        default = "Cargo.toml"
    )]
    pub cargo: String,

    #[input(description = "The Crates API Token", required = true)]
    pub token: String,

    #[input(description = "Crates Registry", default = "https://crates.io")]
    pub registry: String,

    /// List of crates to publish
    #[input(description = "List of crates to publish using names (in order, comma separated)")]
    pub crates: String,

    /// Published version
    #[output(description = "The version of the crate that was published")]
    pub version: String,

    #[output(description = "Version is different from remote version")]
    pub changed: String,
}

impl RustCratePublishAction {
    pub async fn run(&self) -> Result<()> {
        let crates = self.crates();
        log::info!(
            "🏃 Running Rust Crate Publish Action on {} crates",
            crates.len()
        );
        let cargo_path = PathBuf::from(&self.cargo);
        let pwd: PathBuf = match cargo_path.parent() {
            Some(parent) => parent.to_path_buf(),
            None => PathBuf::from("./"),
        };

        let mut cargo = Cargo::read(cargo_path).await?;
        cargo.set_working_directory(pwd);

        if std::env::var("DRY_RUN").is_ok() {
            log::info!("🌵 Dry Run Enabled");
            cargo.dry_run = true;
        }

        for crate_name in crates.iter() {
            let package = cargo.package(crate_name.as_str());

            let name = package.name.clone().unwrap_or_else(|| crate_name.clone());

            log::info!("📦 Crate Name       :: {}", name);

            let Some(local_version) = cargo.version(package) else {
                log::warn!("⚠️  No version found in Cargo.toml");
                continue;
            };

            if local_version.ends_with("-dev") {
                log::warn!("🚨 Local version ends with '-dev'. Skipping...");
                continue;
            }
            if let Some(publish) = package.publish {
                if !publish {
                    log::warn!("🚨 Crate is not set to publish. Skipping...");
                    continue;
                }
            }

            let latest_crate = crates::get_latest(name.to_string()).await?;

            log::info!("🦀 Current version  :: v{}", latest_crate.num);
            log::info!("💻 Local version    :: v{}", local_version);

            self.output_version(&local_version);

            if latest_crate.num == local_version {
                log::info!("🍹 Crate is up to date. Lets sit back and relax...");
                self.output_changed("false".to_string());
                continue;
            }

            cargo.add_registry(self.registry.to_string());

            log::info!("🚀 Crate is out of date. Lets get to work...");
            self.output_changed("true".to_string());

            if !self.token.is_empty() && !cargo.dry_run {
                cargo.login(&self.token).await?;
            }

            cargo.publish(crate_name).await?;
        }

        Ok(())
    }

    fn crates(&self) -> Vec<String> {
        self.crates
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_crates() {
        let input = "foo, bar, baz";

        let action = RustCratePublishAction {
            crates: input.to_string(),
            ..Default::default()
        };
        let crates = action.crates();

        assert_eq!(crates.len(), 3);
        assert_eq!(crates[0], "foo");
        assert_eq!(crates[1], "bar");
        assert_eq!(crates[2], "baz");
    }
}
