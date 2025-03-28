use anyhow::{Context, Result};
use std::path::PathBuf;

/// Cargo configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Cargo {
    /// The main package configuration.
    pub package: Option<Package>,
    /// The workspace configuration.
    pub workspace: Option<Workspace>,

    // Non-standard fields
    #[serde(skip)]
    pub working_directory: PathBuf,
    #[serde(default, skip)]
    pub packages: Vec<Package>,
    #[serde(skip)]
    pub registry: Option<String>,
    #[serde(skip)]
    pub dry_run: bool,
}

/// Workspace configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Workspace {
    pub members: Vec<String>,

    pub package: Option<Package>,
}

/// Package configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Package {
    pub name: Option<String>,
    pub version: Version,
    pub publish: Option<bool>,
}

/// Version can be a string or a workspace struct (version.workspace = true)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Version {
    Version(String),
    Workspace { workspace: bool },
}

impl Cargo {
    pub async fn read(path: impl Into<PathBuf>) -> Result<Cargo> {
        let path = path.into();
        log::debug!("Reading Cargo.toml from {}", &path.display());

        let data = tokio::fs::read_to_string(&path).await?;
        let mut cargo: Cargo = toml::from_str(&data)?;

        if let Some(Workspace { members, .. }) = &cargo.workspace {
            let root = path.parent().context("Failed to get parent directory")?;
            for member in members.iter() {
                log::debug!("Reading Cargo.toml from `{}`", member);
                let member_path = root.join(member).join("Cargo.toml");

                let member_data = tokio::fs::read_to_string(member_path)
                    .await
                    .context("Failed to read cargo file")?;
                let member_cargo: Cargo =
                    toml::from_str(&member_data).context("Failed to parse cargo file")?;
                cargo.packages.push(member_cargo.package.unwrap());
            }
            log::debug!("Found {} packages", cargo.packages.len());
            log::debug!("Packages: {:#?}", cargo.packages);
        }

        Ok(cargo)
    }

    pub fn package(&self, name: &str) -> &Package {
        if name.is_empty() {
            if let Some(package) = &self.package {
                package
            } else {
                // Assumes if there is no package, there is a workspace
                &self.packages[0]
            }
        } else {
            self.packages
                .iter()
                .find(|&p| p.name == Some(name.to_string()))
                .expect("Package not found")
        }
    }

    /// Gets the version from the package or the workspace.
    pub fn version(&self, package: &Package) -> Option<String> {
        if let Version::Version(version) = &package.version {
            Some(version.to_string())
        } else if let Version::Workspace { workspace } = &package.version {
            if *workspace {
                log::debug!("Looking for workspace version");
                match &self.workspace {
                    Some(Workspace {
                        package:
                            Some(Package {
                                version: Version::Version(version),
                                ..
                            }),
                        ..
                    }) => {
                        log::debug!("Found workspace version: {}", version);
                        Some(version.to_string())
                    }
                    _ => {
                        log::warn!("No workspace version found");
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set_working_directory(&mut self, working_directory: PathBuf) {
        log::debug!(
            "Setting working directory to {}",
            working_directory.display()
        );
        self.working_directory = working_directory;
    }

    /// Sets the registry to use for the cargo commands.
    pub fn add_registry(&mut self, registry: String) {
        log::debug!("Setting registry to {}", registry);
        self.registry = Some(registry);
    }

    /// Authenticate with the registry.
    pub async fn login(&self, token: &String) -> Result<()> {
        let mut args = vec!["login".to_string()];

        args.push(token.to_string());

        let output = tokio::process::Command::new("cargo")
            .args(args)
            .current_dir(&self.working_directory)
            .output()
            .await
            .context("Failed to run cargo login")?;

        if output.status.success() {
            log::info!("🔑 Successfully logged into Crates.io");
            Ok(())
        } else {
            log::error!(
                "🔑 Failed to login to Crates.io\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
            Err(anyhow::anyhow!("Failed to login to Crates.io"))
        }
    }

    /// Publish the crate to the registry.
    pub async fn publish(&self, name: &String) -> Result<()> {
        log::info!("🚀 Publishing crate to registry");
        let mut args = vec!["publish".to_string()];

        if self.dry_run {
            log::info!("⛷️  Dry run enabled...");
            args.push("--dry-run".to_string());
        }

        if self.workspace.is_some() {
            args.push("-p".to_string());
            args.push(name.to_string());
        }
        args.push("--allow-dirty".to_string());

        log::debug!("Publishing crate with args: {:?}", args);

        let output = tokio::process::Command::new("cargo")
            .args(args)
            .current_dir(&self.working_directory)
            .output()
            .await
            .context("Failed to run cargo publish")?;
        if output.status.success() {
            log::info!("🚀 Successfully published to Crates.io");
        } else {
            log::error!(
                "🚀 Failed to publish to Crates.io\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
            return Err(anyhow::anyhow!("Failed to publish to Crates.io"));
        }

        Ok(())
    }
}
