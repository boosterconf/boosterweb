---
title: "Aligning Bounded Contexts with Subdomains in Legacy Code"
talk_type: "Experience report (30 min)"
authors:
    - Mufrid Krilic
time_slot: "Thursday: 14:15 - 14:45"
room: "Kongesalen 4"
vimeo_id: 692597907
---
One way or another, each system contains some kind of boundaries. I would go so far and claim that even the dreaded Big Ball of Mud systems consist of parts that could be perceived as separate though undoubtedly only under deep scrutiny. The difference is in the “thickness” of the boundaries and the measure of interrelationships between the different parts of the system, the frequency and amount of data that is passed across the fences. It is the latter that leads to increased coupling resulting in systems that are hard to maintain and hard to change.

This presentation will present a story of an attempt to achieve an alignment between perceived subdomains, logical boundaries and source code structure in a legacy system. Based on the use case from healthcare we will go into technical detail on concrete steps that were followed to create a new bounded context using strategic Domain-Driven Design and 4+1 Architectural View Models.
