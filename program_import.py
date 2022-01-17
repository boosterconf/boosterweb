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

with open('talks.csv', 'rt', encoding="utf-8") as csvfile:
    reader = csv.reader(csvfile, quotechar='"')
    for row in reader:
        # number email navn copresentername copresentermail norsk title abstract type j k l m n o p q r s t u v w x akseptert z
        number = row[0]
        navn = row[2]
        copresentername = row[3]
        title = row[6]
        abstract = row[7]
        akseptert = row[24]
        if akseptert == "x":
            with open("content/talk/" + number + "-" + string_to_filename(title) + ".md", "w+") as file:
                file.truncate(0)
                file.write('---\n')
                file.write(f'title: "{title}"\n')
                file.write('authors:\n')
                file.write(f'    - {navn}\n')
                if copresentername != "":
                    file.write(f'    - {copresentername}\n')

                file.write('---\n')
            create_speaker_file(navn)
            if copresentername != "":
                create_speaker_file(copresentername)
                    