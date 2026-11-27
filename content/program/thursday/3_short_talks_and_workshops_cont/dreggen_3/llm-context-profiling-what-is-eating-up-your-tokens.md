---
title: "LLM Context Profiling: what is eating up your tokens?"
talk_type: "Workshop 1,5h"
type: talk
starts_at: 2027-03-18T10:50:00Z
ends_at: 2027-03-18T12:20:00Z
weight: 1
authors:
    - Rimantas Zukaitis

---
One of the most painful aspects of AI use is the ever-increasing bills for token usage - especially with providers reducing subscription limits or switching to token-based billing. This is especially true for AI coding agents - here input token use can be 20 to 50 times higher than actual generated LLM output and often accounts for a major part of the cost. Lots of attention is spent on trying to find a "silver bullet" - on frameworks or tools to reduce token usage automatically, with varying levels of success.

In this session, we take a different, manual approach. We will explore what goes into the context window, how to analyse and profile its contents using a custom-built profiler tool, and how token use can be reduced without losing precision. It's not a replacement, but an addition to existing optimisation tools - think of memory or CPU profiling, but applied to an LLM context. Just like a snapshot makes finding performance bottlenecks easier, the ability to review and understand the contents of the context window helps identify unnecessary, duplicated, or low-value information before it affects cost, latency, and model performance.
