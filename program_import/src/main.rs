mod domain;
mod presentation;
mod sessionize;
mod utils;

use std::path::Path;

/// Quite stripped down. Natural place for argument parsing, config, and other
/// stuff that isn't needed right now
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let days = sessionize::fetch_program().await?;

    println!("Importing Booster 2026 program");
    let target_dir = Path::new("../content/program");
    presentation::days_to_markdown(target_dir.into(), days)?;

    Ok(())
}
