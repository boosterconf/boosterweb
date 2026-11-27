---
title: "From Ship to Shore: Designing OrcaWeb End to End"
talk_type: "Experience report"
type: talk
starts_at: 2027-03-19T09:00:00Z
ends_at: 2027-03-19T09:30:00Z
weight: 1
authors:
    - Florian Pujol
    - Carl August Gjørsvik

---
What does a web platform look like when some users are at sea, connectivity is unreliable, and operational data must also support analytics and regulatory reporting?

This architecture experience report draws on our work designing OrcaWeb at Odfjell. Following a vessel reporting workflow, I will present the target platform end to end: a React application with feature-scoped offline support, real-time updates, a backend-for-frontend, domain APIs, and a Microsoft Fabric data platform feeding analytics and regulatory report generation.

Attendees will see where we place responsibilities and why: local capture versus shared state, operational APIs versus analytical data products, and business calculations versus report generation and lifecycle management.

Rather than touring every service, we will examine the design decisions behind these boundaries and the consequences of getting them wrong. The lessons apply to any team connecting occasionally disconnected users, enterprise integrations, and downstream consumers that need trustworthy data.
