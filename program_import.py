# coding=utf-8
from pathlib import Path
import os


schedule_url = "https://sessionize.com/api/v2/dblm5kaq/view/GridSmart"
all_url = "https://sessionize.com/api/v2/dblm5kaq/view/All"

from urllib.request import urlopen 
from urllib.parse import urlparse
from dateutil import parser
   
import json
import frontmatter
import datetime
import pytz
import re



all_response = urlopen(all_url) 
all_json = json.loads(all_response.read())

sessions_map = {}

for session in all_json['sessions']:
    sessions_map[session['title']] = session

session_formats = {}
session_tags = {}
session_accepted_as = {}

for session_format in all_json['categories'][0]['items']:
    session_formats[session_format['id']] = session_format
for session_tag in all_json['categories'][1]['items']:
    session_tags[session_tag['id']] = session_tag
for accepted_as in all_json['categories'][2]['items']:
    if(accepted_as['name'] == "Special Workshop"):
        accepted_as['name'] = "Workshop"
    session_accepted_as[accepted_as['id']] = accepted_as


speaker_map = {}

for speaker in all_json['speakers']:
    speaker_map[speaker['id']] = speaker

response = urlopen(schedule_url) 
data_json = json.loads(response.read())


predefined_slots = {
    'wednesday': [],
    'thursday': [],
    'friday': [],
}

room_weights = {
}

def parse_slot(path):
    fm = frontmatter.load(path)
    time = fm['time'].split(" - ")
    slot = {
        #'title': fm['title'],
        'start': time[0],
        'end':  time[1] if len(time) > 1 else time[0],
        'sessions': [],
    }
    return slot

for day in predefined_slots:
    for day_slot in os.listdir("content/program/" + day):
        path = "content/program/" + day + "/" + day_slot
        if os.path.isdir(path):
            print("parsing slot as room " + "content/program/" + day + "/" + day_slot + "/_index.md")
            slot = parse_slot("content/program/" + day + "/" + day_slot + "/_index.md")
            slot['isRoom'] = True
            slot['path'] = "content/program/" + day + "/" + day_slot
            predefined_slots[day].append(slot)
        else:
            print("parsing slot as non-room " + "content/program/" + day + "/" + day_slot)
            if day_slot != "_index.md":
                slot = parse_slot("content/program/" + day + "/" + day_slot)
                slot['isRoom'] = False
                slot['path'] = "content/program/" + day + "/" + day_slot
                predefined_slots[day].append(slot)

tz = pytz.timezone('CET')
# print(predefined_slots)
for day in data_json:
    day_start = parser.parse(day["date"])
    day_predefined_slots = predefined_slots[day_start.strftime("%A").lower()]
    print(day_start.strftime("%A").lower())
    weight = 0
    for room in day['rooms']:
        room_weights[room['name']] = weight = weight + 1
        for session in room['sessions']:
            session_start = parser.parse(session['startsAt'])
            session_end = parser.parse(session['endsAt'])
            found_slot = False
            for day_predefined_slot in day_predefined_slots:

                (slot_start_hr, slot_start_m) = day_predefined_slot['start'].split(":")
                (slot_end_hr, slot_end_m) = day_predefined_slot['end'].split(":")
                slot_start = session_start.replace(hour=int(slot_start_hr)-1, minute=int(slot_start_m)) # CET -> UTC included
                slot_end = session_end.replace(hour=int(slot_end_hr)-1, minute=int(slot_end_m)) # CET -> UTC included
                day_predefined_slot['startDateTime'] = slot_start
                day_predefined_slot['endDateTime'] = slot_end
                
                session_starts_in_slot = (session_start >= slot_start and session_start < slot_end)
                session_starts_before_and_ends_after_slot = (session_start < slot_start and session_end > slot_end)
                session_ends_in_slot = (session_end <= slot_end and session_end > slot_start)
                if session_starts_in_slot or session_ends_in_slot or session_starts_before_and_ends_after_slot:
                    found_slot = True
                    day_predefined_slot['sessions'].append({'sessionize': session, 'continuation': (session_ends_in_slot and not session_starts_in_slot) or session_starts_before_and_ends_after_slot})
            if not found_slot:
                print("Could not find slot for session " + session['title'])

def write_room_header(path, session):
    sessionize = session['sessionize']
    room = sessionize['room']
    room_key = room.replace(" + ", "_").replace(" ", "_").replace("-","_").lower()
    room_exists = os.path.exists(path + "/" + room_key)
    if(room == "Kongesalen 1-4"):
        room_name = "Kongesalen plenum"
    else:
        room_name = room
    session_data = sessions_map[sessionize['title']]
    language = ""
    for questionAnswers in session_data['questionAnswers']:
        if questionAnswers['questionId'] == 62991:
            language = questionAnswers['answerValue']
    if not room_exists:
        os.makedirs(path + "/" + room_key)
    room_index_path = path + "/" + room_key + "/_index.md"
    f = open(room_index_path, "w", encoding="utf-8")
    template = f"""---
title: "{room_name}"
type: room
language: {language.lower()}
weight: {room_weights[room]}
---"""
    f.write(template)
    return path + "/" + room_key
def write_session_file(room_path, session, weight):
    sessionize = session['sessionize']
    session_title_key = re.sub("[^a-z0-9æøå]", "-", sessionize['title'].lower())
    session_title_key = re.sub("-+", "-", session_title_key)
    session_title_key = re.sub("(^-)*(-$)*", "", session_title_key)

    session_path = room_path + "/" + session_title_key + ".md"
#    if not room_exists:
#        os.makedirs(path + "/" + room_key)
#        room_index_path = path + "/" + room_key + "/_index.md"
    session_data = sessions_map[sessionize['title']]
    if not session_data['isServiceSession']:
        session_format = [session_accepted_as[x] for x in session_data['categoryItems'] if x in session_accepted_as][0]['name']
        print(session_format)
        #session_data['categories'][0]['categoryItems'][0]['name']
        session_title = re.sub("\"", "\\\"", sessionize['title'])
        recording_url_formatted = ""
        if session_data['recordingUrl']:
            recording_url_formatted = f"\nrecording_url: {session_data['recordingUrl']}"
        speakers_formatted = ""
        for speaker_id in session_data['speakers']:
            speaker = speaker_map[speaker_id]
            speakers_formatted += "    - " + speaker['firstName'] + " " + speaker['lastName'] + "\n"

        template = f"""---
title: "{session_title}"
talk_type: "{session_format}"
type: talk{recording_url_formatted}
starts_at: {sessionize['startsAt']}
ends_at: {sessionize['endsAt']}
weight: {weight}
authors:
{speakers_formatted}
---
{session_data['description']}
"""
        f = open(session_path, "wt", encoding="utf-8", newline="\n")
        f.write(template)
#    return path + "/" + room_key

def write_session_continuation(room_path, session, weight):
    sessionize = session['sessionize']
    room = sessionize['room']

    session_title = re.sub("\"", "\\\"", sessionize['title'])
    session_data = sessions_map[sessionize['title']]
    if not session_data['isServiceSession']:
        print(session_data['categoryItems'])
        session_format = [session_accepted_as[x] for x in session_data['categoryItems'] if x in session_accepted_as][0]['name']
        language = ""
        for questionAnswers in session_data['questionAnswers']:
            if questionAnswers['questionId'] == 62991:
                language = questionAnswers['answerValue']

        template = f"""---
title: "Continues: {session_title}"
talk_type: "{session_format}"
type: talk_continuation
weight: {weight}
---
"""
        session_path = room_path + "/continuation.md"

        f = open(session_path, "w", encoding="utf-8", newline="\n")
        f.write(template)

for day in predefined_slots:
    day_predefined_slots = predefined_slots[day]
    for day_predefined_slot in day_predefined_slots:
        weight = 1
        for session in day_predefined_slot['sessions']:
            if day_predefined_slot['isRoom']:
                room = session['sessionize']['room']
                print("Creating session "+ session['sessionize']['title'] +" in room " + room + " weight: (" + str(room_weights[room]) + ")")
                room_path = write_room_header(day_predefined_slot['path'], session)
                
                if session['continuation']:
                    write_session_continuation(room_path, session, weight)
                else:
                    write_session_file(room_path, session, weight)
                weight = weight+1

for speaker_id in speaker_map:
    print("creating speaker " + (speaker['firstName'] + " " + speaker['lastName']))
    speaker = speaker_map[speaker_id]
    speaker_name = re.sub("\"", "\\\"", (speaker['firstName'] + " " + speaker['lastName']))
    speaker_title =  re.sub("\"", "\\\"", speaker['tagLine'] or "")
    template = f"""---
name: "{speaker_name}"
title: "{speaker_title}"
# twitter_handle: 
---
{speaker['bio']}
"""

    speaker_key = re.sub("[^a-z0-9æøå]", "-", speaker_name.lower())
    speaker_key = re.sub("-+", "-", speaker_key)
    speaker_key = re.sub("(^-)*(-$)*", "", speaker_key)
    speaker_dir = "content/speaker/" + speaker_key
    speaker_exists = os.path.exists(speaker_dir)
    if not speaker_exists:
        os.makedirs(speaker_dir)
    f = open(speaker_dir + "/index.md", "w", encoding="utf-8", newline="\n")
    f.write(template)
    profile_picture_url = speaker['profilePicture']
    if profile_picture_url != None:
        url = urlparse(profile_picture_url)
        filename = os.path.basename(url.path)
        profile_picture_path = speaker_dir + "/" + filename
        file_exists = os.path.exists(profile_picture_path)
        if not file_exists:
            print("loading image " + profile_picture_url)
            data = urlopen(profile_picture_url)
            open(profile_picture_path, "wb", newline="\n").write(data.read())
        

    
# for day in data_json:
#     day_start = parser.parse(day["date"])
#     if start_new_slot:
#         print("" + day_start.strftime("%A"))
#     for slot in day["timeSlots"]:
#         if start_new_slot:
#             print("\t" + slot["slotStart"])
#         start_new_slot = False
#         for room in slot["rooms"]:
#             session = room["session"]
#             if session["isServiceSession"]:
#                 print("" + day_start.strftime("%A"))
#                 print("\t" + slot["slotStart"])
#                 start_new_slot = True
#             print("\t\t" + room["name"])
#             print("\t\t\t" + session["title"])

#day_dir = Path(x[15].value.strftime("content/program/%A").lower())
#day_dir.mkdir(parents=True, exist_ok=True)