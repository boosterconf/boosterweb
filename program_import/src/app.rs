use std::io::{Write, stdout};

use itertools::Itertools;

use crate::{
    domain::{Day, Speaker, is_session_category_content},
    files::{ProgramFiles, SlotOrNot, SpeakerFiles},
    presentation::{
        day_markdown, room_markdown, session_markdown, slot_markdown, speaker_markdown,
    },
    sessionize,
    utils::tree_term,
};

pub async fn days_to_markdown(
    program_files: ProgramFiles,
    days: Vec<Day>,
) -> Result<(), Box<dyn std::error::Error>> {
    program_files.create_program_base_dir().await?;

    let mut day_iter = days.iter().enumerate().peekable();

    while let Some((day_i, day)) = day_iter.next() {
        let contents = day_markdown(day_i, day);
        program_files.save_day(day, contents).await?;

        let last_day = day_iter.peek().is_none();
        println!("{}{}", tree_term(vec![last_day]), day.date.format("%A"));

        let mut slot_iter = day.time_slots.iter().enumerate().peekable();
        while let Some((slot_i, slot)) = slot_iter.next() {
            let slot_i = slot_i + 1;

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
                let md = session_markdown(slot_i, &slot.rooms[0].sessions[0], &slot.rooms[0]);
                program_files
                    .save_session(day, SlotOrNot::Not(slot_i), &slot.rooms[0].sessions[0], md)
                    .await?;

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
                let contents = slot_markdown(slot_i, slot);
                program_files.save_slot(slot_i, day, slot, contents).await?;

                let mut room_iter = slot.rooms.iter().enumerate().peekable();
                while let Some((room_i, room)) = room_iter.next() {
                    let room_md = room_markdown(room_i + 1, room);

                    program_files
                        .save_room(day, slot_i, slot, room, room_md)
                        .await?;

                    let last_room = room_iter.peek().is_none();
                    println!(
                        "{}{}",
                        tree_term(vec![last_day, last_slot, last_room]),
                        room.name,
                    );

                    let mut sessions_iter = room.sessions.iter().enumerate().peekable();
                    while let Some((session_i, session)) = sessions_iter.next() {
                        let session_md = session_markdown(session_i + 1, session, room);
                        program_files
                            .save_session(
                                day,
                                SlotOrNot::Slot(slot, slot_i, room),
                                session,
                                session_md,
                            )
                            .await?;
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

pub async fn speakers_to_markdown(
    speaker_files: &SpeakerFiles,
    speakers: &[Speaker],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut removed_speakers = speaker_files.existing_speakers(speakers).await?;
    removed_speakers.retain(|x| !speakers.contains(x));

    speaker_files.create_speakers_base_dir().await?;

    for speaker in speakers.iter() {
        let content = speaker_markdown(speaker);
        speaker_files.save_speaker(speaker, content).await?;
    }

    for speaker in removed_speakers {
        println!("Removed speaker: {}", &speaker.name);
        speaker_files.remove_speaker(speaker.name).await?;
    }

    Ok(())
}

pub async fn speakers_to_profile_pictures(
    speaker_files: &SpeakerFiles,
    speakers: &[Speaker],
) -> Result<(), Box<dyn std::error::Error>> {
    speaker_files.create_profile_picture_base_dir().await?;

    let existing_profile_pictures = speaker_files.existing_profile_pictures().await?;

    let grouped_existing_profile_pictures = existing_profile_pictures
        .iter()
        .into_group_map_by(|x| x.1.clone());

    for pics in grouped_existing_profile_pictures.values() {
        if pics.len() > 1 {
            for pic in pics {
                speaker_files.remove_profile_picture(&pic.0, &pic.1).await?;
            }
        }
    }

    let speakers_with_pics = speakers
        .iter()
        .filter(|x| {
            if let Some(pic) = &x.profile_picture {
                !existing_profile_pictures.iter().any(|x| &x.0 == pic)
            } else {
                false
            }
        })
        .collect::<Vec<_>>();
    let total = speakers_with_pics.len();

    for (i, speaker) in speakers_with_pics.iter().enumerate() {
        if let Some(pic) = &speaker.profile_picture {
            if !existing_profile_pictures.iter().any(|x| &x.0 == pic) {
                print!("\rDownloading speaker photos ({i:03}/{total:03})");
                stdout().flush()?;

                let bytes = sessionize::fetch_profile_picture(pic).await?;
                speaker_files
                    .save_profile_picture(pic, speaker, bytes)
                    .await?;
            }
        }
    }

    if total > 0 {
        println!();
    } else {
        println!("No speaker photos to download");
    }

    Ok(())
}
