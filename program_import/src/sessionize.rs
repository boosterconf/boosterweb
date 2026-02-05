//! Sessionize client, data structures, and conversions to domain data
//! structures

use crate::domain::*;

use chrono::{DateTime, NaiveTime, Utc};
use itertools::Itertools;
use serde::Deserialize;
use tokio::join;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionizeRoom {
    name: String,
    session: SessionizeSession,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionizeSpeaker {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionizeSession {
    id: String,
    title: String,
    is_service_session: bool,
    is_plenum_session: bool,
    starts_at: DateTime<Utc>,
    ends_at: DateTime<Utc>,
    description: Option<String>,
    categories: Vec<SessionizeCategory>,
    speakers: Vec<SessionizeSpeaker>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionizeCategoryItem {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionizeCategory {
    name: String,
    category_items: Vec<SessionizeCategoryItem>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionizeSlot {
    slot_start: NaiveTime,
    rooms: Vec<SessionizeRoom>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionizeDay {
    date: DateTime<Utc>,
    time_slots: Vec<SessionizeSlot>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionizeAllMetadata {
    sessions: Vec<SessionizeSessionMetadata>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionizeSessionMetadata {
    id: String,
    question_answers: Vec<SessionizeQuestionAnswers>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionizeQuestionAnswers {
    question_id: usize,
    #[serde(deserialize_with = "crate::utils::deserialize_bool_from_str")]
    answer_value: bool,
}

impl TryFrom<(SessionizeSession, &SessionizeSessionMetadata)> for Session {
    type Error = String;

    fn try_from(
        (value, metadata): (SessionizeSession, &SessionizeSessionMetadata),
    ) -> Result<Self, Self::Error> {
        let category = SessionCategory::try_from(&value)?;

        Ok(Session {
            title: value.title,
            is_service_session: value.is_service_session,
            is_plenum_session: value.is_plenum_session,
            is_english: metadata
                .question_answers
                .iter()
                .find(|x| x.question_id == 114013)
                .is_some_and(|x| x.answer_value),
            is_continuation: false,
            starts_at: value.starts_at,
            ends_at: value.ends_at,
            description: value.description,
            category,
            speakers: value
                .speakers
                .into_iter()
                .map(|x| x.name)
                .collect::<Vec<_>>(),
        })
    }
}

impl TryFrom<&SessionizeSession> for SessionCategory {
    type Error = String;

    fn try_from(session: &SessionizeSession) -> Result<Self, Self::Error> {
        match session.title.as_str() {
            "Registration" => Ok(SessionCategory::Registration),
            "Open spaces" => Ok(SessionCategory::OpenSpaces),
            "Welcome from the organizers" => Ok(SessionCategory::ConferenceIntro),
            "Lunch" => Ok(SessionCategory::Lunch),
            "Break" => Ok(SessionCategory::Break),
            "Conference dinner" => Ok(SessionCategory::Dinner),
            "Speaker's dinner" => Ok(SessionCategory::Dinner),
            "To be announced" => Ok(SessionCategory::ToBeAnnounced),
            "Day ends" => Ok(SessionCategory::DayEnds),
            _ => {
                let accepted_as = session
                    .categories
                    .iter()
                    .filter(|x| x.name == "Akseptert som")
                    .collect::<Vec<_>>();

                if accepted_as.len() != 1 {
                    return Err(format!(
                        "All session categories that are not based on title name must have one single \"Akseptert som\" category, {}, does not",
                        session.title
                    ));
                }

                let category_items = &accepted_as[0].category_items;

                if category_items.len() != 1 {
                    return Err(format!(
                        "All \"Akseptert som\" categories must have one single category item, {}, does not",
                        session.title
                    ));
                }

                match category_items[0].name.as_str() {
                    "Keynote" => Ok(SessionCategory::Keynote),
                    "Experience Report" => Ok(SessionCategory::ExperienceReport),
                    "Lightning Talk" => Ok(SessionCategory::LightningTalk),
                    "Workshop 1,5h" => Ok(SessionCategory::Workshop90),
                    "Workshop 3h" => Ok(SessionCategory::Workshop180),
                    x => Err(format!(
                        "Couldn't figure out category on {}. The 'accepted as' category was: {}",
                        session.title, x
                    )),
                }
            }
        }
    }
}

impl TryFrom<(&SessionizeAllMetadata, &SessionizeDayToDayIntermediate)> for Session {
    type Error = String;

    fn try_from(
        (metadata, intermediate): (&SessionizeAllMetadata, &SessionizeDayToDayIntermediate),
    ) -> Result<Self, Self::Error> {
        Ok(Session {
            title: intermediate.room.session.title.clone(),
            is_service_session: intermediate.room.session.is_service_session,
            is_plenum_session: intermediate.room.session.is_plenum_session,
            is_english: metadata
                .sessions
                .iter()
                .find(|y| y.id == intermediate.room.session.id)
                .unwrap()
                .question_answers
                .iter()
                .find(|x| x.question_id == 114013)
                .is_some_and(|x| x.answer_value),
            is_continuation: intermediate.continuation,
            starts_at: intermediate.room.session.starts_at,
            ends_at: intermediate.room.session.ends_at,
            description: intermediate.room.session.description.clone(),
            category: SessionCategory::try_from(&intermediate.room.session)?,
            speakers: intermediate
                .room
                .session
                .speakers
                .iter()
                .cloned()
                .map(|x| x.name)
                .collect::<Vec<_>>(),
        })
    }
}

/// Intermediate struct used while converting a SessionizeDay to a proper Day
struct SessionizeDayToDayIntermediate {
    continuation: bool,
    room: SessionizeRoom,
}

impl
    TryFrom<(
        &SessionizeAllMetadata,
        Vec<SessionizeDayToDayIntermediate>,
        Option<DateTime<Utc>>,
    )> for Slot
{
    type Error = String;

    fn try_from(
        (metadata, intermediates, ends_at): (
            &SessionizeAllMetadata,
            Vec<SessionizeDayToDayIntermediate>,
            Option<DateTime<Utc>>,
        ),
    ) -> Result<Self, Self::Error> {
        let new_rooms = intermediates
            .into_iter()
            .into_group_map_by(|x| x.room.name.clone())
            .into_iter()
            .map(|x| Room {
                name: x.0,
                sessions: x
                    .1
                    .into_iter()
                    .map(|x| Session::try_from((metadata, &x)).unwrap())
                    .collect::<Vec<_>>(),
            })
            .sorted()
            .collect::<Vec<_>>();

        Ok(Slot {
            starts_at: new_rooms
                .first()
                .unwrap()
                .sessions
                .first()
                .unwrap()
                .starts_at,
            ends_at: ends_at.unwrap_or_else(|| {
                new_rooms
                    .iter()
                    .flat_map(|x| &x.sessions)
                    .map(|x| x.ends_at)
                    .max()
                    .unwrap()
            }),
            rooms: new_rooms,
        })
    }
}

/// An awkwardly complex transform from `SessionizeDay` to `Day`. We need two
/// sets of data for this for the current approach. An alternative would be not
/// relying on Sessionize slots, but this works without _too much_ awkwardness.
///
/// Single slot single room non content sessions are used to chunk up the
/// Sessionize slots to match the ones we use. Continuation sessions are also
/// created here.
impl TryFrom<(SessionizeDay, &SessionizeAllMetadata)> for Day {
    type Error = String;

    fn try_from(
        (day, metadata): (SessionizeDay, &SessionizeAllMetadata),
    ) -> Result<Self, Self::Error> {
        let mut current_intermediates: Vec<SessionizeDayToDayIntermediate> = Vec::new();
        let mut slots: Vec<Slot> = Vec::new();

        for slot in day.time_slots.into_iter().peekable() {
            if let [non_content_room] = slot.rooms.as_slice()
                && let category = SessionCategory::try_from(&non_content_room.session)?
                && !is_session_category_content(&category)
            {
                if !current_intermediates.is_empty() {
                    let x = std::mem::take(&mut current_intermediates);
                    let mut continuations = x
                        .iter()
                        .filter(|x| x.room.session.ends_at > non_content_room.session.starts_at)
                        .map(|x| SessionizeDayToDayIntermediate {
                            room: x.room.clone(),
                            continuation: true,
                        })
                        .collect::<Vec<_>>();

                    current_intermediates.append(&mut continuations);

                    slots.push(Slot::try_from((
                        metadata,
                        x,
                        Some(non_content_room.session.starts_at),
                    ))?);
                };

                slots.push(Slot {
                    starts_at: day.date.with_time(slot.slot_start).unwrap(),
                    ends_at: non_content_room.session.ends_at,
                    rooms: vec![Room {
                        name: non_content_room.name.clone(),
                        sessions: vec![Session::try_from((
                            metadata,
                            &SessionizeDayToDayIntermediate {
                                room: non_content_room.clone(),
                                continuation: false,
                            },
                        ))?],
                    }],
                });
            } else {
                current_intermediates.append(
                    &mut slot
                        .rooms
                        .into_iter()
                        .map(|x| SessionizeDayToDayIntermediate {
                            continuation: false,
                            room: x,
                        })
                        .collect::<Vec<_>>(),
                );
            }
        }

        if !current_intermediates.is_empty() {
            slots.push(Slot::try_from((
                metadata,
                std::mem::take(&mut current_intermediates),
                None,
            ))?);
        }

        Ok(Day {
            date: day.date.date_naive(),
            time_slots: slots,
        })
    }
}

/// Parse data from Sessionize into domain data structures
fn program_parse(
    grid_smart: String,
    all_metadata: String,
) -> Result<Vec<Day>, Box<dyn std::error::Error>> {
    let days: Vec<SessionizeDay> = serde_json::from_str(&grid_smart)?;
    let all: SessionizeAllMetadata = serde_json::from_str(&all_metadata)?;
    let days = days
        .into_iter()
        .map(|x| Day::try_from((x, &all)))
        .collect::<Result<Vec<Day>, String>>()?;

    Ok(days)
}

/// Fetch `GridSmart` and `All` JSONs from Sessionize API, parse them, and
/// return the result as domain data structures
pub async fn fetch_program() -> Result<Vec<Day>, Box<dyn std::error::Error>> {
    let (grid_smart, all_metadata) = join!(
        async {
            Ok::<_, reqwest::Error>(
                reqwest::get("https://sessionize.com/api/v2/1gnvn8rl/view/GridSmart")
                    .await?
                    .text()
                    .await?,
            )
        },
        async {
            Ok::<_, reqwest::Error>(
                reqwest::get("https://sessionize.com/api/v2/1gnvn8rl/view/All")
                    .await?
                    .text()
                    .await?,
            )
        },
    );

    program_parse(grid_smart?, all_metadata?)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use chrono::{NaiveDate, TimeZone};

    use super::*;

    #[test]
    fn test_program_parse() {
        let days = fs::read_to_string("test_fixtures/GridSmart.json").unwrap();
        let all = fs::read_to_string("test_fixtures/All.json").unwrap();

        let mut days = program_parse(days, all).unwrap();
        days[1].time_slots.truncate(1);
        days[1].time_slots[0].rooms.truncate(1);
        days[1].time_slots[0].rooms[0].sessions.truncate(1);

        assert_eq!(
            // We're only testing Thursday as it's the most complex day
            days[1],
            Day {
                date: NaiveDate::from_ymd_opt(2026, 3, 12).unwrap(),
                time_slots: vec![Slot {
                    starts_at: Utc.with_ymd_and_hms(2026, 3, 12, 8, 0, 0).unwrap(),
                    ends_at: Utc.with_ymd_and_hms(2026, 3, 12, 9, 30, 0).unwrap(),
                    rooms: vec![Room {
                        name: "Kongesalen 2-3".to_string(),
                        sessions: vec![Session {
                            title: "There is more to life than KPIs".to_string(),
                            is_english: true,
                            is_continuation: false,
                            is_service_session: false,
                            is_plenum_session: false,
                            starts_at: Utc.with_ymd_and_hms(2026, 3, 12, 8, 0, 0).unwrap(),
                            ends_at: Utc.with_ymd_and_hms(2026, 3, 12, 8, 30, 0).unwrap(),
                            description: Some("During an Open Space discussion at last year`s Booster Conference, we discussed the distance between leaders and teams. The gap is real, but it is not there because of bad intentions. Instead, I think the gap exists because it is difficult to see how leaders achieve control by letting go of it. Over the past year leading an area of highly skilled and diverse teams, I have put this into practice. Some things worked, others didn’t, but the consistent theme is that I, as a leader, must prove myself worthy of the team's trust. Perhaps these insights can provide value to others as well?".to_string()),
                            category: SessionCategory::ExperienceReport,
                            speakers: vec!["Louis Dieffenthaler".to_string()],
                        }]
                    }]
                }],
            }
        )
    }
}
