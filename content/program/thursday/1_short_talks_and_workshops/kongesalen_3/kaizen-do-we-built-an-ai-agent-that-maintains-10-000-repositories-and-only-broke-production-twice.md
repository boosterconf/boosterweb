---
title: "kaizen-do: We Built an AI Agent That Maintains 10,000 Repositories and Only Broke Production Twice"
talk_type: "Experience report"
type: talk
starts_at: 2027-03-18T10:00:00Z
ends_at: 2027-03-18T10:30:00Z
weight: 3
authors:
    - Yves Hwang

---
You know that backlog of CVEs, outdated dependencies, and security findings nobody wants to touch? We built an multi-agent orchestrated system to do it for us. kaizen-do runs four scanners in parallel, triages findings with LLMs, and opens pull requests with fixes, with optional human in the loop. It routes high confidence fixes straight to PRs and low confidence ones to issues. It deduplicates across scanners, caps PR volume with backpressure so teams don't drown, and learns from closed PRs to get smarter over time. Built in four weeks by eight engineers from DNB and AWS, with a shared ambition: open source it so any org can point it at their own repos. Go, Step Functions, Bedrock, and a state machine we're oddly proud of. This talk walks through the architecture, the failures that taught us the most, and why a bank and a cloud provider decided the best way to solve this problem was to give the solution away.
