//! File repository. All file and path handling.

use std::path::{Path, PathBuf};

use bytes::Bytes;
use itertools::Itertools;
use regex::Regex;
use tokio::fs::{self};
use tokio_stream::{StreamExt, wrappers::ReadDirStream};

use crate::domain::{
    Day, ProfilePicture, Room, Session, SessionCategory, Slot, Speaker, is_session_category_content,
};

fn session_category_to_dir_names(cat: &SessionCategory) -> &'static str {
    match cat {
        SessionCategory::Break => "break",
        SessionCategory::Lunch => "lunch",
        SessionCategory::Keynote => "keynote",
        SessionCategory::Registration => "registration",
        SessionCategory::DayEnds => "done",
        SessionCategory::ToBeAnnounced => "to_be_announced",
        SessionCategory::Dinner => "conference_dinner",
        SessionCategory::ConferenceIntro => "welcome",
        SessionCategory::OpenSpaces => "open_spaces",
        SessionCategory::LightningTalk => "lightning_talks",
        SessionCategory::ExperienceReport => "short_talks",
        SessionCategory::Workshop90 => "workshops",
        SessionCategory::Workshop180 => "workshops",
        SessionCategory::SpecialWorkshop => "workshops",
        SessionCategory::Plenum => "plenum",
    }
}

fn slot_dir_name(slot: &Slot) -> String {
    let mut dir_name = slot
        .rooms
        .iter()
        .flat_map(|x| &x.sessions)
        .map(|x| session_category_to_dir_names(&x.category))
        .sorted()
        .dedup()
        .join("_and_");

    if slot
        .rooms
        .iter()
        .flat_map(|x| &x.sessions)
        .any(|x| x.is_continuation)
    {
        dir_name.push_str("_cont");
    }

    dir_name
}

pub struct ProgramFiles {
    base_dir: PathBuf,
}

/// Argument to `save_session`. Either a `Slot`, `usize` and `Room` to calculate
/// sub directories, or just a usize to make a top level file with numbered
/// prefix
pub enum SlotOrNot<'a> {
    Slot(&'a Slot, usize, &'a Room),
    Not(usize),
}

impl ProgramFiles {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub async fn create_program_base_dir(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(fs::create_dir_all(&self.base_dir).await?)
    }

    fn day_dir(&self, day: &Day) -> PathBuf {
        let day_name = day.date.format("%A");
        self.base_dir.join(day_name.to_string().to_lowercase())
    }

    pub async fn save_day(
        &self,
        day: &Day,
        contents: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let day_dir = Self::day_dir(self, day);

        fs::remove_dir_all(&day_dir).await?;
        fs::create_dir(&day_dir).await?;
        fs::write(day_dir.join("_index.md"), contents).await?;

        Ok(())
    }

    fn slot_dir(&self, day: &Day, slot: &Slot, i: usize) -> PathBuf {
        let day_dir = Self::day_dir(self, day);

        let dir_name = slot_dir_name(slot);
        day_dir.join(format!("{}_{}", i, dir_name))
    }

    pub async fn save_slot(
        &self,
        i: usize,
        day: &Day,
        slot: &Slot,
        contents: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let slot_dir = Self::slot_dir(self, day, slot, i);

        fs::create_dir(&slot_dir).await?;
        fs::write(slot_dir.join("_index.md"), contents).await?;

        Ok(())
    }

    fn room_dir(&self, day: &Day, slot: &Slot, slot_i: usize, room: &Room) -> PathBuf {
        let slot_dir = Self::slot_dir(self, day, slot, slot_i);

        slot_dir.join(
            room.name
                .to_string()
                .to_lowercase()
                .replace(" ", "_")
                .replace("-", "_"),
        )
    }

    pub async fn save_room(
        &self,
        day: &Day,
        slot_i: usize,
        slot: &Slot,
        room: &Room,
        contents: String,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let room_dir = Self::room_dir(self, day, slot, slot_i, room);

        fs::create_dir(&room_dir).await?;
        fs::write(room_dir.join("_index.md"), contents).await?;

        Ok(room_dir)
    }

    fn session_filename(i: Option<usize>, session: &Session) -> String {
        let filename = if session.is_continuation {
            "continuation".to_string()
        } else if is_session_category_content(&session.category) {
            let only_ascii_norwegian = Regex::new(r"[^a-z0-9æøå]").unwrap();
            let only_one_dash_in_a_row = Regex::new(r"--*").unwrap();
            let leading_dash = Regex::new(r"^-").unwrap();
            let lagging_dash = Regex::new(r"-$").unwrap();
            let filename = session.title.to_lowercase();
            let filename = only_ascii_norwegian.replace_all(&filename, "-");
            let filename = only_one_dash_in_a_row.replace_all(&filename, "-");
            let filename = leading_dash.replace(&filename, "");
            let filename = lagging_dash.replace(&filename, "");
            filename.to_string()
        } else {
            session_category_to_dir_names(&session.category).to_string()
        };

        let prefix = i.map_or_else(|| "".to_string(), |x| format!("{x}_"));
        format!("{prefix}{filename}.md")
    }

    pub async fn save_session(
        &self,
        day: &Day,
        slot_or_not: SlotOrNot<'_>,
        session: &Session,
        contents: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let day_name = day.date.format("%A");
        let day_dir = self.base_dir.join(day_name.to_string().to_lowercase());

        let (dir, filename) = match slot_or_not {
            SlotOrNot::Slot(slot, slot_i, room) => {
                let dir_name = slot_dir_name(slot);
                let slot_dir = day_dir.join(format!("{}_{}", slot_i, dir_name));

                let dir = slot_dir.join(
                    room.name
                        .to_string()
                        .to_lowercase()
                        .replace(" ", "_")
                        .replace("-", "_"),
                );

                (dir, Self::session_filename(None, session))
            }
            SlotOrNot::Not(slot_i) => (day_dir, Self::session_filename(Some(slot_i), session)),
        };

        let path = dir.join(filename);

        fs::write(path, contents).await?;

        Ok(())
    }
}

pub struct SpeakerFiles {
    base_dir: PathBuf,
}

impl SpeakerFiles {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Existing speakers on the filesystem
    pub async fn existing_speakers(
        &self,
        speakers: &[Speaker],
    ) -> Result<Vec<Speaker>, Box<dyn std::error::Error>> {
        let files = fs::read_dir(&self.base_dir).await?;
        let files = ReadDirStream::new(files);
        let existing_speaker_paths: Vec<PathBuf> = files
            .filter_map(|x| x.ok())
            .map(|x| x.path())
            .filter(|x| x.is_dir())
            .collect()
            .await;

        Ok(speakers
            .iter()
            .filter(|x| {
                existing_speaker_paths.contains(&Self::create_speaker_path(&self.base_dir, &x.name))
            })
            .cloned()
            .collect::<Vec<_>>())
    }

    pub async fn create_speakers_base_dir(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(fs::create_dir_all(&self.base_dir).await?)
    }

    pub async fn remove_speaker(
        &self,
        speaker_name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::create_speaker_path(&self.base_dir, &speaker_name);
        Ok(fs::remove_dir_all(path).await?)
    }

    fn create_speaker_path(parent_dir: &Path, speaker_name: &str) -> PathBuf {
        parent_dir.join(speaker_name.replace(" ", "-").to_lowercase())
    }

    pub async fn save_speaker(
        &self,
        speaker: &Speaker,
        content: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::create_speaker_path(&self.base_dir, &speaker.name);
        fs::create_dir_all(&path).await?;
        fs::write(path.join("index.md"), content).await?;
        Ok(())
    }

    pub async fn existing_profile_pictures(
        &self,
    ) -> Result<Vec<(ProfilePicture, String)>, Box<dyn std::error::Error>> {
        let mut res = Vec::new();

        let name_regex = Regex::new(r#"name: "(.+)""#).unwrap();

        let mut speaker_dirs = fs::read_dir(&self.base_dir).await?;
        while let Some(speaker_dir_entry) = speaker_dirs.next_entry().await? {
            if speaker_dir_entry.file_type().await?.is_dir() {
                let mut speaker_files = fs::read_dir(&speaker_dir_entry.path()).await?;
                while let Some(f) = speaker_files.next_entry().await? {
                    if let Some(ext) = f.path().extension()
                        && ["jpg", "png"].contains(&ext.to_str().unwrap())
                    {
                        let speaker_file =
                            fs::read_to_string(speaker_dir_entry.path().join("index.md")).await?;
                        let speaker_name = &name_regex.captures(&speaker_file).unwrap()[1];

                        res.push((
                            ProfilePicture {
                                id: format!(
                                    "https://sessionize.com/image/{}",
                                    f.file_name().into_string().unwrap(),
                                ),
                            },
                            speaker_name.to_string(),
                        ));
                    }
                }
            }
        }

        Ok(res)
    }

    pub async fn create_profile_picture_base_dir(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(fs::create_dir_all(&self.base_dir).await?)
    }

    fn profile_picture_path(&self, pic: &ProfilePicture, speaker_name: &str) -> PathBuf {
        let url = &pic.id;
        let file_name = url.rsplit('/').next().unwrap();
        let path = Self::create_speaker_path(&self.base_dir, speaker_name);
        path.join(file_name)
    }

    pub async fn save_profile_picture(
        &self,
        pic: &ProfilePicture,
        speaker: &Speaker,
        bytes: Bytes,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = Self::profile_picture_path(self, pic, &speaker.name);

        fs::write(file_path, bytes).await?;

        Ok(())
    }

    pub async fn remove_profile_picture(
        &self,
        pic: &ProfilePicture,
        speaker_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = Self::profile_picture_path(self, pic, speaker_name);

        dbg!(&file_path);
        Ok(fs::remove_file(file_path).await?)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::domain::{Session, SessionCategory};

    use super::*;

    #[test]
    fn test_slot_dir_name() {
        let dir_name = slot_dir_name(&Slot {
            starts_at: Utc.with_ymd_and_hms(2026, 3, 11, 8, 0, 0).unwrap(),
            ends_at: Utc.with_ymd_and_hms(2026, 3, 11, 9, 0, 0).unwrap(),
            rooms: vec![
                Room {
                    name: "Test room 1".to_string(),
                    sessions: vec![Session {
                        title: "Test talk 1".to_string(),
                        is_english: false,
                        is_continuation: false,
                        is_service_session: false,
                        is_plenum_session: false,
                        starts_at: Utc.with_ymd_and_hms(2026, 3, 12, 8, 0, 0).unwrap(),
                        ends_at: Utc.with_ymd_and_hms(2026, 3, 12, 8, 30, 0).unwrap(),
                        description: Some("A test description".to_string()),
                        category: SessionCategory::ExperienceReport,
                        speakers: vec!["Test speaker".to_string()],
                    }],
                },
                Room {
                    name: "Test room 2".to_string(),
                    sessions: vec![Session {
                        title: "Test talk 2".to_string(),
                        is_english: false,
                        is_continuation: false,
                        is_service_session: false,
                        is_plenum_session: false,
                        starts_at: Utc.with_ymd_and_hms(2026, 3, 12, 8, 30, 0).unwrap(),
                        ends_at: Utc.with_ymd_and_hms(2026, 3, 12, 9, 0, 0).unwrap(),
                        description: Some("A test description".to_string()),
                        category: SessionCategory::ExperienceReport,
                        speakers: vec!["Test speaker".to_string()],
                    }],
                },
                Room {
                    name: "Test room 2".to_string(),
                    sessions: vec![Session {
                        title: "Test workshop".to_string(),
                        is_english: false,
                        is_continuation: true,
                        is_service_session: false,
                        is_plenum_session: false,
                        starts_at: Utc.with_ymd_and_hms(2026, 3, 12, 9, 0, 0).unwrap(),
                        ends_at: Utc.with_ymd_and_hms(2026, 3, 12, 9, 30, 0).unwrap(),
                        description: Some("A test description".to_string()),
                        category: SessionCategory::Workshop90,
                        speakers: vec!["Test speaker".to_string()],
                    }],
                },
            ],
        });
        assert_eq!(dir_name, "short_talks_and_workshops_cont".to_string());
    }

    #[test]
    fn test_session_markdown_talk() {
        let session = Session {
            title: "Test workshop".to_string(),
            is_english: false,
            is_continuation: false,
            is_service_session: false,
            is_plenum_session: false,
            starts_at: Utc.with_ymd_and_hms(2026, 3, 12, 8, 0, 0).unwrap(),
            ends_at: Utc.with_ymd_and_hms(2026, 3, 12, 9, 30, 0).unwrap(),
            description: Some("A test description".to_string()),
            category: SessionCategory::Workshop90,
            speakers: vec![
                "Speaky Speakerson".to_string(),
                "Worky Workshoppy".to_string(),
            ],
        };

        let filename = ProgramFiles::session_filename(Some(1), &session);

        assert_eq!(filename, ("1_test-workshop.md".to_string()))
    }
}
