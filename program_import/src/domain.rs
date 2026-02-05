//! The Booster domain data structures and functions.

use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, PartialEq, Eq)]
pub struct Day {
    pub date: NaiveDate,
    pub time_slots: Vec<Slot>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Slot {
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub rooms: Vec<Room>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Room {
    pub name: String,
    pub sessions: Vec<Session>,
}

impl PartialOrd for Room {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Room {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .sessions
            .len()
            .cmp(&self.sessions.len())
            .then(self.name.cmp(&other.name))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Session {
    pub title: String,
    pub is_service_session: bool,
    pub is_plenum_session: bool,
    /// Is this the continuation of a session that span one or more breaks?
    /// Should be in a view layer.
    pub is_continuation: bool,
    pub is_english: bool,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub description: Option<String>,
    pub category: SessionCategory,
    /// Speakers are assumed to have unique names across the whole site.
    /// Sessionize has IDs for when we need to handle duplicates
    pub speakers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionCategory {
    Break,
    Keynote,
    Registration,
    DayEnds,
    Lunch,
    Dinner,
    ToBeAnnounced,
    ConferenceIntro,
    OpenSpaces,
    LightningTalk,
    ExperienceReport,
    Workshop90,
    Workshop180,
}

pub fn is_session_category_content(session_category: &SessionCategory) -> bool {
    match session_category {
        SessionCategory::Break => false,
        SessionCategory::Lunch => false,
        SessionCategory::Registration => false,
        SessionCategory::ConferenceIntro => false,
        SessionCategory::DayEnds => false,
        SessionCategory::ToBeAnnounced => true,
        SessionCategory::Dinner => false,
        SessionCategory::Keynote => true,
        SessionCategory::OpenSpaces => true,
        SessionCategory::LightningTalk => true,
        SessionCategory::ExperienceReport => true,
        SessionCategory::Workshop90 => true,
        SessionCategory::Workshop180 => true,
    }
}

pub fn session_has_end(session_category: &SessionCategory) -> bool {
    match session_category {
        SessionCategory::Break => true,
        SessionCategory::Lunch => true,
        SessionCategory::Registration => true,
        SessionCategory::ConferenceIntro => true,
        SessionCategory::DayEnds => false,
        SessionCategory::ToBeAnnounced => true,
        SessionCategory::Dinner => false,
        SessionCategory::Keynote => true,
        SessionCategory::OpenSpaces => true,
        SessionCategory::LightningTalk => true,
        SessionCategory::ExperienceReport => true,
        SessionCategory::Workshop90 => true,
        SessionCategory::Workshop180 => true,
    }
}
