---
title: "Drowning in Code You Didn't Write: Comprehension Debt in Agentic Development"
talk_type: "Experience report"
type: talk
starts_at: 2027-03-18T11:50:00Z
ends_at: 2027-03-18T12:20:00Z
weight: 3
authors:
    - Dmitrijs Lebedevs

---
AI coding assistants have delivered on their promise: we are writing code faster than ever before. But speed comes with a hidden, dangerous cost. Over the past year of building agentic systems and distributed architectures, I discovered a new metric of failure: Comprehension Debt.

Comprehension debt occurs when an AI generates 500 lines of perfectly functioning code in seconds, but the human developer fundamentally lacks the mental model of how it works. When the code inevitably breaks—or when business logic changes—the developer is left debugging a "black box" they never truly understood.

In this experience report, I will share the painful lessons learned while building complex AI systems (including Zero Trust proxies and trading bots) using AI coding agents (Claude Code, Cursor). I’ll discuss how my team initially drowned in generated spaghetti code, and how we recovered by fundamentally changing our workflow.

I will introduce practical strategies to manage Comprehension Debt, shifting from prompt-and-pray coding to Specification-Driven Development. We will explore how to use Architectural Decision Records (ADRs), structured CLAUDE.md context files, and AI-assisted reverse-engineering to ensure that humans stay firmly in the driver's seat of the architectural mental model.
