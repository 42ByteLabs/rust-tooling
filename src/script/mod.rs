use anyhow::Result;
use ghactions::Actions;
use std::path::PathBuf;

#[derive(Actions, Debug, Default)]
#[action(
    name = "rust-tooling-script",
    description = "Rust Tooling Script",
    path = "./script/action.yml",
    image = "./script/Dockerfile"
)]
pub struct RustToolingScriptAction {
    /// The in-line script content to run
    #[input(description = "The in-line script content to run")]
    pub script: String,

    /// The path to the script file to run
    #[input(description = "The path to the script file to run")]
    pub path: String,

    #[input(description = "The Crates API Token", required = true)]
    pub token: String,
}

impl RustToolingScriptAction {
    pub async fn run(&self) -> Result<()> {
        log::info!("🏃 Running Rust Tooling Script Action");

        let path = if !self.path.is_empty() {
            let script_path = PathBuf::from(&self.path);
            log::debug!("Script Path: {}", script_path.display());

            if !script_path.exists() {
                return Err(anyhow::anyhow!("Script file does not exist"));
            } else if !script_path.is_file() {
                return Err(anyhow::anyhow!("Script file is not a file"));
            }

            script_path
        } else if !self.script.is_empty() {
            let script_path = PathBuf::from("script.rs");

            let mut data = self.script.clone();

            if !data.starts_with("#!") {
                data = format!("#!/usr/bin/env rust-script\n{}", data);
            }

            tokio::fs::write(&script_path, &self.script).await?;

            script_path
        } else {
            return Err(anyhow::anyhow!("Either script or path must be provided"));
        };
        log::info!("📄 Script Path: {}", path.display());

        let mut cmd = vec!["rust-script"];

        cmd.push(path.as_os_str().to_str().unwrap());

        let mut output = tokio::process::Command::new("sh")
            .args(cmd)
            .output()
            .await?;

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout).unwrap();
            log::info!("📄 Script Output: {}", stdout);
        } else {
            let stderr = String::from_utf8(output.stderr).unwrap();
            log::error!("📄 Script Error: {}", stderr);
            return Err(anyhow::anyhow!("Script failed"));
        }

        Ok(())
    }
}
