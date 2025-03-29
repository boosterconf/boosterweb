import vimeo
import webbrowser
import multiprocessing

import vimeo.upload
from werkzeug import Request, Response, run_simple
import readline
from prompt_toolkit.completion import Completer, Completion
from prompt_toolkit.shortcuts import CompleteStyle, prompt, message_dialog, yes_no_dialog
import json
from urllib.request import urlopen 
import os


# OAuth provider configuration
client_id = 'dd23bd2fd84248e2c34c467963a4e7236d640085'
authorization_base_url = 'https://api.vimeo.com/oauth/authorize'
token_url = 'https://api.vimeo.com/oauth/token'
redirect_uri = 'http://localhost:5975/callback'
state = ''
scope_list = 'upload'
event_id = '15043'

webbrowser.open('https://api.vimeo.com/oauth/authorize?response_type=token&response_mode=form_post&client_id={client_id}&redirect_uri={redirect_uri}&state={state}&scope={scope_list}'.format(client_id= client_id,redirect_uri= redirect_uri,state=state,scope_list=scope_list))

def run_token_server(q: multiprocessing.Queue) -> None:
    @Request.application
    def app(request: Request) -> Response:
        token = request.args.get('access_token', None)
        if(token is None):
            return Response('<!DOCTYPE html><html lang="en"><head><title>Document</title></head><body><script>var fragment = window.location.hash.substring(1); window.location="http://localhost:5975/fragment_callback?"+fragment;</script></body></html>', status=200, content_type='text/html')
        q.put(token)
        return Response('Token captured. You can close this window now.', 200)

    run_simple("localhost", 5975, app)

def get_token():
    q = multiprocessing.Queue()
    p = multiprocessing.Process(target=run_token_server, args=(q,))
    p.start()
    token = q.get(block=True)
    p.terminate()
    return token



access_token = get_token()
print('access_token:', access_token) 
client = vimeo.VimeoClient(
  token=access_token,
  key='dd23bd2fd84248e2c34c467963a4e7236d640085'
)


class TitleCompleter(Completer):
    def __init__(self, all_json):
        self.loading = 0
        self.titles = [session['title'] for session in all_json['sessions']]

    def get_completions(self, document, complete_event):
        # Keep count of how many completion generators are running.
        self.loading += 1
        word_before_cursor = document.get_word_before_cursor()

        try:
            for title in self.titles:
                if title.startswith(word_before_cursor):
                    yield Completion(title, -len(word_before_cursor))
                    
        finally:
            # We use try/finally because this generator can be closed if the
            # input text changes before all completions are generated.
            self.loading -= 1

def load_sessionize_data():
    all_url = "https://sessionize.com/api/v2/dblm5kaq/view/All"
    all_response = urlopen(all_url) 
    return json.loads(all_response.read())
sessionize_data = load_sessionize_data()

def upload_file(file_name, title):
    try:
        # Upload the file and include the video title and description.
        uri = client.upload(file_name, data={
            'name': title
        })

        # Get the metadata response from the upload and log out the Vimeo.com url
        video_data = client.get(uri + '?fields=link').json()
        print('"{}" has been uploaded to {}'.format(file_name, video_data['link']))
        link = video_data['link']
        # Make an API call to see if the video is finished transcoding.
        video_data = client.get(uri + '?fields=transcode.status').json()
        print('The transcode status for {} is: {}'.format(
            uri,
            video_data['transcode']['status']
        ))
        return link
    except vimeo.exceptions.VideoUploadFailure as e:
        # We may have had an error. We can't resolve it here necessarily, so
        # report it to the user.
        print('Server reported: %s' % e.message)

def show_file(file_name):
    if platform.system() == 'Darwin':       # macOS
        subprocess.call(('open', file_name))
    elif platform.system() == 'Windows':    # Windows
        os.startfile(file_name)
    else:                                   # linux variants
        subprocess.call(('xdg-open', file_name))

import subprocess, os, platform
from pathlib import Path
import pyperclip

files = list(Path("upload").rglob("*.mp4"))
    
title_completer = TitleCompleter(sessionize_data)
for file in files:
    filename = os.fsdecode(file)
    show_file(filename)
#    if(not yes_no_dialog(
#        title='Continue?',
#        text='Do you want to upload ' + filename).run()):
#        continue
    title = prompt('Talk title? ', completer=title_completer)
    session = [session for session in sessionize_data['sessions'] if session['title'] == title][0]
    speaker_names = [speaker['firstName'] +' '+ speaker['lastName'] for speaker in sessionize_data['speakers'] if speaker['id'] in session['speakers']]
    if(session['recordingUrl'] is None):
        uri = upload_file(filename, session['title'] + ' - ' + ' & '.join(speaker_names) )
        pyperclip.copy(uri)
        webbrowser.open('https://sessionize.com/app/organizer/session/edit/{event_id}/{session_id}?recordingUrl=#Session_RecordingUrl'.format(event_id = event_id, session_id = session['id']))
        prompt('URI is copied to clipboard. Proceed with next video?')
        
    continue



