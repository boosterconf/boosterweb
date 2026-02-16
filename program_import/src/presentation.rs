//! Presentation layer. Which right now means Markdown for Hugo to consume.

use std::{fs, path::PathBuf};

use chrono::Datelike;
use chrono_tz::Europe::Oslo;
use itertools::Itertools;
use regex::Regex;

use crate::{
    domain::{
        Day, Room, Session, SessionCategory, Slot, is_session_category_content, session_has_end,
    },
    utils::{number_ordinal, tree_term},
};

pub fn days_to_markdown(
    target_dir: PathBuf,
    days: Vec<Day>,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(&target_dir)?;

    let mut day_iter = days.iter().enumerate().peekable();

    while let Some((i, day)) = day_iter.next() {
        let day_name = day.date.format("%A");

        let day_dir = target_dir.join(day_name.to_string().to_lowercase());
        let _ = fs::remove_dir_all(&day_dir);
        let _ = fs::create_dir(&day_dir);
        let day_md = day_markdown(i, day);
        let _ = fs::write(day_dir.join("_index.md"), day_md);

        let last_day = day_iter.peek().is_none();
        println!("{}{}", tree_term(vec![last_day]), day.date.format("%A"));

        let mut slot_iter = day.time_slots.iter().enumerate().peekable();
        while let Some((i, slot)) = slot_iter.next() {
            let i = i + 1;

            let needs_dir = if let &[room] = &slot.rooms.as_slice()
                && let &[session] = &room.sessions.as_slice()
            {
                is_session_category_content(&session.category)
            } else {
                true
            };

            let last_slot = slot_iter.peek().is_none();
            println!(
                "{}{}-{}",
                tree_term(vec![last_day, last_slot]),
                slot.starts_at.format("%H:%M"),
                slot.ends_at.format("%H:%M")
            );

            if !needs_dir {
                let (session_filename, md) =
                    session_markdown(i, &slot.rooms[0].sessions[0], &slot.rooms[0]);
                let path = day_dir.join(format!("{i}_{session_filename}"));

                fs::write(path, md)?;

                println!(
                    "{}{}",
                    tree_term(vec![last_day, last_slot, true]),
                    slot.rooms[0].name,
                );
                println!(
                    "{}{} - {}",
                    tree_term(vec![last_day, last_slot, true, true]),
                    slot.rooms[0].sessions[0].title,
                    slot.rooms[0].sessions[0].is_plenum_session,
                );
            } else {
                let dir_name = slot_dir_name(slot);
                let slot_dir = day_dir.join(format!("{}_{}", i, dir_name));
                let _ = fs::create_dir(&slot_dir);

                let contents = slot_markdown(i, slot);
                let _ = fs::write(slot_dir.join("_index.md"), contents);

                let mut room_iter = slot.rooms.iter().enumerate().peekable();
                while let Some((i, room)) = room_iter.next() {
                    let room_dir = slot_dir.join(
                        room.name
                            .to_string()
                            .to_lowercase()
                            .replace(" ", "_")
                            .replace("-", "_"),
                    );
                    let _ = fs::create_dir(&room_dir);

                    let room_md = room_markdown(i + 1, room);
                    let _ = fs::write(room_dir.join("_index.md"), room_md);

                    let last_room = room_iter.peek().is_none();
                    println!(
                        "{}{}",
                        tree_term(vec![last_day, last_slot, last_room]),
                        room.name,
                    );

                    let mut sessions_iter = room.sessions.iter().enumerate().peekable();
                    while let Some((i, session)) = sessions_iter.next() {
                        let (session_filename, session_md) = session_markdown(i + 1, session, room);
                        let _ = fs::write(room_dir.join(session_filename), session_md);

                        let last_session = sessions_iter.peek().is_none();
                        println!(
                            "{}{} - {}",
                            tree_term(vec![last_day, last_slot, last_room, last_session]),
                            session.title,
                            session.is_plenum_session,
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

fn content_session_md_type(session: &Session) -> (&'static str, Option<&'static str>) {
    match session.category {
        SessionCategory::Break => ("break", None),
        SessionCategory::Lunch => ("break", None),
        SessionCategory::ConferenceIntro => ("welcome", None),
        SessionCategory::Registration => ("registration", None),
        SessionCategory::DayEnds => ("period", None),
        SessionCategory::ToBeAnnounced => ("talk", None),
        SessionCategory::OpenSpaces => ("talk", Some("Open Spaces")),
        SessionCategory::Dinner => ("period", None),
        SessionCategory::Keynote => ("talk", Some("Keynote")),
        SessionCategory::LightningTalk => ("talk", Some("Lightning Talk")),
        SessionCategory::ExperienceReport => ("talk", Some("Experience report")),
        SessionCategory::Workshop90 => ("talk", Some("Workshop 1,5h")),
        SessionCategory::Workshop180 => ("talk", Some("Workshop 3h")),
    }
}

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

fn slot_markdown(i: usize, slot: &Slot) -> String {
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

fn day_markdown(i: usize, day: &Day) -> String {
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

fn room_markdown(i: usize, room: &Room) -> String {
    let title = &room.name;
    let lang = if room.sessions.iter().any(|x| x.is_english) {
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

fn session_markdown(i: usize, session: &Session, room: &Room) -> (String, String) {
    let title = session.title.replace("\"", "\\\"");
    let md_types = content_session_md_type(session);
    let type_ = md_types.0;

    let (filename, content) = if session.is_continuation {
        let talk_type = md_types.1.unwrap_or("");

        let md = format!(
            r#"---
title: "Continues: {title}"
talk_type: "{talk_type}"
type: {type_}
weight: {i}
---
"#
        );

        ("continuation.md".to_string(), md)
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

        let md = format!(
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
        );

        let only_ascii_norwegian = Regex::new(r"[^a-z0-9æøå]").unwrap();
        let only_one_dash_in_a_row = Regex::new(r"--*").unwrap();
        let leading_dash = Regex::new(r"^-").unwrap();
        let lagging_dash = Regex::new(r"-$").unwrap();
        let filename = session.title.to_lowercase();
        let filename = only_ascii_norwegian.replace_all(&filename, "-");
        let filename = only_one_dash_in_a_row.replace_all(&filename, "-");
        let filename = leading_dash.replace(&filename, "");
        let filename = lagging_dash.replace(&filename, "");
        (format!("{}.md", filename), md)
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

        let filename = session_category_to_dir_names(&session.category);

        let md = format!(
            r#"---
time: "{time}"{location}
title: "{title}"
type: {type_override}
weight: {i}
---{description}
"#
        );

        (format!("{}.md", filename), md)
    };

    (filename, content)
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, TimeZone, Utc};

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

        assert_eq!(
            // We're only testing Thursday as it's the most complex day
            md,
            ("test-workshop.md".to_string(), expected_md.to_string())
        )
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

        assert_eq!(
            // We're only testing Thursday as it's the most complex day
            md,
            ("break.md".to_string(), expected_md.to_string())
        )
    }
}
