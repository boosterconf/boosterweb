//! Presentation layer. Which right now means Markdown for Hugo to consume.

use chrono::Datelike;
use chrono_tz::Europe::Oslo;

use crate::domain::{
    Day, Room, Session, SessionCategory, Slot, Speaker, is_session_category_content,
    session_has_end,
};

/// English number ordinals
fn number_ordinal(num: u32) -> &'static str {
    match num % 100 {
        11..=13 => "th",
        x => match x % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    }
}

fn content_session_md_type(session: &Session) -> (&'static str, Option<&'static str>) {
    match session.category {
        SessionCategory::Break => ("break", None),
        SessionCategory::Lunch => ("lunch", None),
        SessionCategory::ConferenceIntro => ("welcome", None),
        SessionCategory::Registration => ("registration", None),
        SessionCategory::DayEnds => ("break", None),
        SessionCategory::ToBeAnnounced => ("talk", None),
        SessionCategory::OpenSpaces => ("talk", Some("Open Spaces")),
        SessionCategory::Dinner => ("period", None),
        SessionCategory::Keynote => ("talk", Some("Keynote")),
        SessionCategory::LightningTalk => ("talk", Some("Lightning Talk")),
        SessionCategory::ExperienceReport => ("talk", Some("Experience report")),
        SessionCategory::Workshop90 => ("talk", Some("Workshop 1,5h")),
        SessionCategory::Workshop180 => ("talk", Some("Workshop 3h")),
        SessionCategory::SpecialWorkshop => ("talk", Some("Workshop")),
        SessionCategory::Plenum => ("talk", Some("Plenum")),
    }
}

pub fn slot_markdown(i: usize, slot: &Slot) -> String {
    let starts_at = slot.starts_at.with_timezone(&Oslo).format("%H:%M");
    let ends_at = slot.ends_at.with_timezone(&Oslo).format("%H:%M");

    let content = format!(
        r#"---
time: {starts_at} - {ends_at}
type: period
weight: {i}
---
"#
    );

    content
}

pub fn day_markdown(i: usize, day: &Day) -> String {
    let i = i + 1;
    let day_name = day.date.format("%A");
    let day_name_short = day.date.format("%a");
    let day_of_month = format!("{}{}", day.date.day(), number_ordinal(day.date.day()));
    let month = day.date.format("%B");

    let content = format!(
        r#"---
title: "{day_name} {day_of_month} {month}"
weight: {i}
type: programday
menu:
    main:
        weight: {i}
        parent: "program"
        name: "{day_name_short}"
---
"#
    );

    content
}

pub fn room_markdown(i: usize, room: &Room) -> String {
    let title = &room.name;
    let lang = if room.sessions.iter().all(|x| x.is_english) {
        "english"
    } else {
        ""
    };

    format!(
        r#"---
title: "{title}"
type: room
language: {lang}
weight: {i}
---
"#
    )
}

pub fn session_markdown(i: usize, session: &Session, room: &Room) -> String {
    let title = session.title.replace("\"", "\\\"");
    let md_types = content_session_md_type(session);
    let type_ = md_types.0;

    if session.is_continuation {
        let talk_type = md_types.1.unwrap_or("");

        format!(
            r#"---
title: "Continues: {title}"
talk_type: "{talk_type}"
type: {type_}
weight: {i}
---
"#
        )
    } else if is_session_category_content(&session.category) {
        let talk_type = md_types.1.unwrap_or("");
        let starts_at = session
            .starts_at
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let ends_at = session
            .ends_at
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let authors = match session.category {
            SessionCategory::ToBeAnnounced => "".to_string(),
            SessionCategory::OpenSpaces => "    - You!".to_string(),
            _ => format!("- {}", session.speakers.join("\n    - ")),
        };
        let description = session
            .description
            .as_deref()
            .unwrap_or("")
            .replace("\r\n", "\n");

        format!(
            r#"---
title: "{title}"
talk_type: "{talk_type}"
type: {type_}
starts_at: {starts_at}
ends_at: {ends_at}
weight: {i}
authors:
    {authors}

---
{description}
"#
        )
    } else {
        let starts_at = session
            .starts_at
            .with_timezone(&Oslo)
            .time()
            .format("%H:%M");
        let ends_at = session.ends_at.with_timezone(&Oslo).time().format("%H:%M");
        let time = if session_has_end(&session.category) {
            format!("{starts_at} - {ends_at}")
        } else {
            format!("{starts_at}")
        };
        let location = if session.is_plenum_session {
            "".to_string()
        } else {
            format!("\nlocation: \"{}\"", room.name)
        };
        let description = session
            .description
            .clone()
            .map_or("".to_string(), |x| format!("\n{x}"));

        // Type is period for some reason on a few session categories. Can clean up later by fixing in Hugo.
        let type_override = match session.category {
            SessionCategory::Dinner => "period",
            SessionCategory::Keynote => "period",
            SessionCategory::Registration => "period",
            SessionCategory::ConferenceIntro => "period",
            _ => type_,
        };

        format!(
            r#"---
time: "{time}"{location}
title: "{title}"
type: {type_override}
weight: {i}
---{description}
"#
        )
    }
}

pub fn speaker_markdown(speaker: &Speaker) -> String {
    let name = &speaker.name;
    let title = speaker
        .title
        .clone()
        .map_or_else(|| "".to_string(), |x| format!("\ntitle: \"{x}\""));
    let bio = speaker
        .bio
        .clone()
        .map_or_else(|| "".to_string(), |x| format!("\n{x}"));
    format!(
        r#"---
name: "{name}"{title}
---{bio}
"#
    )
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, TimeZone, Utc};

    use super::*;

    #[test]
    fn test_number_ordinal() {
        assert_eq!("st".to_string(), number_ordinal(101));
        assert_eq!("nd".to_string(), number_ordinal(102));
        assert_eq!("rd".to_string(), number_ordinal(103));
        assert_eq!("st".to_string(), number_ordinal(1));
        assert_eq!("nd".to_string(), number_ordinal(2));
        assert_eq!("rd".to_string(), number_ordinal(3));
        assert_eq!("th".to_string(), number_ordinal(111));
        assert_eq!("th".to_string(), number_ordinal(112));
        assert_eq!("th".to_string(), number_ordinal(113));
        assert_eq!("th".to_string(), number_ordinal(115));
        assert_eq!("rd".to_string(), number_ordinal(123));
    }

    #[test]
    fn test_day_markdown() {
        let res = day_markdown(
            0,
            &Day {
                date: NaiveDate::from_ymd_opt(2026, 3, 11).unwrap(),
                time_slots: vec![],
            },
        );

        assert_eq!(
            res,
            r#"---
title: "Wednesday 11th March"
weight: 1
type: programday
menu:
    main:
        weight: 1
        parent: "program"
        name: "Wed"
---
"#
            .to_string()
        )
    }

    #[test]
    fn test_room_markdown() {
        let res = room_markdown(
            1,
            &Room {
                name: "Test room 1".to_string(),
                sessions: vec![Session {
                    title: "Test workshop".to_string(),
                    is_english: false,
                    is_continuation: false,
                    is_service_session: false,
                    is_plenum_session: false,
                    starts_at: Utc.with_ymd_and_hms(2026, 3, 12, 7, 30, 0).unwrap(),
                    ends_at: Utc.with_ymd_and_hms(2026, 3, 12, 9, 0, 0).unwrap(),
                    description: Some("A test description".to_string()),
                    category: SessionCategory::Workshop90,
                    speakers: vec!["Test speaker".to_string()],
                }],
            },
        );

        assert_eq!(
            res,
            r#"---
title: "Test room 1"
type: room
language: 
weight: 1
---
"#
            .to_string()
        )
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

        let md = session_markdown(
            1,
            &session,
            &Room {
                name: "Test room".to_string(),
                sessions: vec![],
            },
        );

        let expected_md = r#"---
title: "Test workshop"
talk_type: "Workshop 1,5h"
type: talk
starts_at: 2026-03-12T08:00:00Z
ends_at: 2026-03-12T09:30:00Z
weight: 1
authors:
    - Speaky Speakerson
    - Worky Workshoppy

---
A test description
"#;

        assert_eq!(md, expected_md.to_string())
    }

    #[test]
    fn test_session_markdown_non_talk() {
        let session = Session {
            title: "Break".to_string(),
            is_english: false,
            is_continuation: false,
            is_service_session: false,
            is_plenum_session: false,
            starts_at: Utc.with_ymd_and_hms(2026, 3, 12, 10, 15, 0).unwrap(),
            ends_at: Utc.with_ymd_and_hms(2026, 3, 12, 10, 35, 0).unwrap(),
            description: Some("A test description".to_string()),
            category: SessionCategory::Break,
            speakers: vec![
                "Speaky Speakerson".to_string(),
                "Worky Workshoppy".to_string(),
            ],
        };

        let md = session_markdown(
            1,
            &session,
            &Room {
                name: "Test room".to_string(),
                sessions: vec![],
            },
        );

        let expected_md = r#"---
time: "11:15 - 11:35"
location: "Test room"
title: "Break"
type: break
weight: 1
---
A test description
"#;

        assert_eq!(md, expected_md.to_string())
    }
}
