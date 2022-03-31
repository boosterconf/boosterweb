---
title: "Transpiling our way into the future"
talk_type: "Experience report (30 min)"
authors:
    - Håkon André Førre Knudsen
time_slot: "Friday: 13:30 - 14:00"
room: "Kongesalen 1"
vimeo_id: 692599612
---
The first version of RamBase Cloud ERP was released in 1992. Back then it was running on in-house developed hardware, with all business logic written in an in-house developed programming language named COS. We have since moved on from the custom hardware, but COS has been with us until now.

After almost 30 years in production, COS had gone through a lot of development. The language was optimized to run on our data model, and developer efficacy had always been an important focus. But adding new features and maintaining old ones had become a tedious task. COS also lacked a lot of functionality found in more general-purpose languages, like C#. This, together with increasing requirements of more scalability, showed us that the time had come to move on to both a new runtime environment and a new language for expressing our business logic.

All of RamBases business logic was written in COS. Rewriting everything would take ages and was not an option. We had to find another solution. Maybe we could transpile COS into another language? And that is exactly what we ended up doing.

This is a post-mortem from our crazy journey of transpiling old COS into modern C#.
