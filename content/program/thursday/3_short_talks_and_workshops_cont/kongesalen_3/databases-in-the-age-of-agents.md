---
title: "Databases in the age of agents"
talk_type: "Experience report"
type: talk
starts_at: 2027-03-18T11:20:00Z
ends_at: 2027-03-18T11:50:00Z
weight: 2
authors:
    - Gülçin Yıldırım Jelinek

---
Databases were designed for humans: long-lived connections, predictable traffic and slowly evolving schemas. Agent-driven systems are breaking those assumptions by generating bursty, unpredictable workloads at a scale traditional database design never anticipated.

We'll walk through four emerging interaction patterns: tool-using agents issuing many short queries; evaluation environments requiring fast cloning and strong isolation; autonomous development agents treating the database as disposable infrastructure and agents using databases as hybrid memory stores combining relational, vector and document storage.

These workloads demand primitives traditional platforms were not designed around yet transactional correctness, durability and operational safety remain non-negotiable.

The goal is to examine how agent-driven development is already changing the assumptions databases were built around and what PostgreSQL (the database I am the most experienced of) needs to remain the default platform in that world.

