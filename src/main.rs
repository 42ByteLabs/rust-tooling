use anyhow::Result;

mod action;
mod publish;
mod script;
mod utils;

use action::RustActions;

#[tokio::main]
async fn main() -> Result<()> {
    let action = RustActions::init()?;

    action.run().await?;
    log::info!("✅ Completed");
    Ok(())
}
