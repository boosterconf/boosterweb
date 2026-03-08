mod app;
mod constants;
mod domain;
mod files;
mod presentation;
mod sessionize;

use std::path::Path;

use crate::files::{ProgramFiles, SpeakerFiles};

/// Quite stripped down. Natural place for argument parsing, config, and other
/// stuff that isn't needed right now
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching Sessionize data");
    let sessionize_data = sessionize::fetch_sessionize_data().await?;

    println!("Parsing days and speakers");
    let days = sessionize::program_parse(&sessionize_data.0, &sessionize_data.1)?;
    let speakers = sessionize::speakers_parse(sessionize_data.1)?;

    println!("Writing Booster 2026 program");
    let program_files = ProgramFiles::new(Path::new("../content/program").into());
    app::days_to_markdown(program_files, days).await?;

    println!("Writing Booster 2026 speakers");
    let speaker_files = SpeakerFiles::new(Path::new("../content/speaker").into());
    app::speakers_to_markdown(&speaker_files, &speakers).await?;
    app::speakers_to_profile_pictures(&speaker_files, &speakers).await?;

    println!("Booster 2026 program import complete!");

    Ok(())
}
