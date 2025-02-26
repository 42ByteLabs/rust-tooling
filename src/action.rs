use anyhow::Result;
use clap::{Parser, Subcommand};
use console::style;
use ghactions::prelude::*;

pub const VERSION_NUMBER: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub const BANNER: &str = r#"______          _ _____           _ _
| ___ \        | |_   _|         | (_)
| |_/ /   _ ___| |_| | ___   ___ | |_ _ __   __ _
|    / | | / __| __| |/ _ \ / _ \| | | '_ \ / _` |
| |\ \ |_| \__ \ |_| | (_) | (_) | | | | | | (_| |
\_| \_\__,_|___/\__\_/\___/ \___/|_|_|_| |_|\__, |
                                             __/ |
                                            |___/"#;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// Enable Debugging
    #[clap(long, env, default_value_t = false)]
    pub debug: bool,

    #[clap(subcommand)]
    pub action: Actions,
}

#[derive(Subcommand, Debug)]
pub enum Actions {
    Publish,
}

#[derive(Debug)]
pub enum RustActions {
    /// Publish a Rust Crate
    Publish(crate::publish::RustCratePublishAction),
}

impl RustActions {
    pub fn init() -> Result<Self> {
        let arguments = Arguments::parse();
        println!(
            "{}    {} - v{}",
            style(BANNER).green(),
            style(AUTHOR).red(),
            style(VERSION_NUMBER).blue()
        );

        match arguments.action {
            Actions::Publish => {
                let action = crate::publish::RustCratePublishAction::init()?;
                Ok(RustActions::Publish(action))
            }
        }
    }

    pub async fn run(&self) -> Result<()> {
        match self {
            RustActions::Publish(action) => action.run().await?,
        }
        Ok(())
    }
}
