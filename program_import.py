# coding=utf-8
import csv
import re
import pathlib

def string_to_filename(stuff):
    out = re.sub(r'[^a-zæøå]+', '-', stuff.lower())
    out = re.sub(r'[-]+$', '', out)
    out = re.sub(r'^[-]+', '', out)
    return out
def create_speaker_file(speaker_name):
    pathlib.Path("content/speaker/" + string_to_filename(speaker_name) + "/").mkdir(parents=True, exist_ok=True)
    with open("content/speaker/" + string_to_filename(speaker_name) + "/index.md", "w+") as file:
        file.truncate(0)
        file.write('---\n')
        file.write(f'name: {speaker_name}\n')
        file.write('# title: \n')
        file.write('# twitter_handle: \n')
        file.write('---\n')

talk_type_map = {
    'Workshop 3 hours': "Workshop (3 hours)",
    'Experience report 30 min': "Experience report (30 min)",
    'Workshop 1.5 hour': "Workshop (1.5 hours)",
    'Lightning talk 10 min': "Lightning talk"
}
with open('talks.csv', 'rt', encoding="utf-8") as csvfile:
    reader = csv.reader(csvfile, quotechar='"')
    for row in reader:
        # number email navn copresentername copresentermail norsk title abstract type j k l m n o p q r s t u v w x akseptert z
        number = row[0]
        navn = row[2]
        copresentername = row[3]
        title = row[6]
        abstract = row[7]
        talk_type = row[8]
        akseptert = row[24]
        if akseptert == "x":
            with open("content/talk/" + number + "-" + string_to_filename(title) + ".md", "w+") as file:
                file.truncate(0)
                file.write('---\n')
                file.write(f'title: "{title}"\n')
                file.write(f'talk_type: "{talk_type_map[talk_type]}"\n')
                file.write('authors:\n')
                file.write(f'    - {navn}\n')
                if copresentername != "":
                    file.write(f'    - {copresentername}\n')

                file.write('---\n')
                file.write(abstract + "\n")
            create_speaker_file(navn)
            if copresentername != "":
                create_speaker_file(copresentername)
                    