mod constants;
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
    let speakers = sessionize::fetch_speakers().await?;

    println!("Importing Booster 2026 program");
    let target_dir = Path::new("../content/program");
    presentation::days_to_markdown(target_dir.into(), days)?;

    println!("Importing Booster 2026 speakers");
    let speaker_dir = Path::new("../content/speaker");
    presentation::speakers_to_markdown(speaker_dir.into(), &speakers)?;
    sessionize::download_speaker_photos(speaker_dir.into(), &speakers).await?;

    println!("Booster 2026 program import complete!");

    Ok(())
}
