---
title: "I gave my analog synthesizer a URL"
talk_type: "Experience report"
type: talk
starts_at: 2027-03-19T13:30:00Z
ends_at: 2027-03-19T14:00:00Z
weight: 1
authors:
    - Snorre Magnus Davøen

---
My Prophet Rev2 is a 16-voice analog polysynth with knobs, wood cheeks and lots of soul. Lately it has mostly been sitting idle. It made me sad that such a lovely instrument wasn't being played, so I decided I wanted to share it. Anyone with a browser should be able to play it or listen in! That idea became Galdr.

The synth has no network port. It only speaks MIDI, a protocol from 1983 that sends a few bytes over a five-pin cable. This talk is about how I built the bridge between the web and this analog synth. Move a slider in the browser and a small service turns it into MIDI messages, so the filter sweeps on the real synthesizer. The sound is then streamed back to everyone listening.

Along the way you'll get a crash course in how analog synths make sound: oscillators, filters, envelopes and what a patch really is. You'll also learn how MIDI, and a clever extension called NRPN, lets a computer reach hundreds of synth parameters. And it's all live: I'll play the Rev2 from the browser and finish by layering a short multi-track song, one loop at a time.
