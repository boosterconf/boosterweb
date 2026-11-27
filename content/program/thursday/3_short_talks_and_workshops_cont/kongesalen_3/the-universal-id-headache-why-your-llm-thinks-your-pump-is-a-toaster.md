---
title: "The Universal ID Headache: Why Your LLM Thinks Your Pump is a Toaster"
talk_type: "Experience report"
type: talk
starts_at: 2027-03-18T10:50:00Z
ends_at: 2027-03-18T11:20:00Z
weight: 1
authors:
    - Rustam Mehmandarov

---
A pump on a factory floor has five names. SAP calls it 10-EQ-P-0101. The SCADA system sees P101_AREA10. The maintenance log lists PUMP-0101-A. The digital twin platform uses a UUID.

They are all the same pump. But when an LLM tries to answer questions about it, it sees five different objects. The result? Hallucinations and broken logic.

Even if you aren't building digital twins, the "Identifier Mapping Problem" is universal. Every developer who has integrated two  legacy systems has faced the nightmare of determining if User_123 in System A is truly u-123 in System B.

This talk bridges the gap between academic research and practical engineering. Based on findings from the University of Oslo, we will move beyond simple regex matching and look at:
   * The Taxonomy of Identifiers: How to classify and treat different ID types.
   * Resolution Patterns: Architectural strategies for mapping IDs across distributed systems without creating a monolith.
   * The "Golden Record" Myth: Why a single source of truth is often impossible, and how to manage the chaos instead.
