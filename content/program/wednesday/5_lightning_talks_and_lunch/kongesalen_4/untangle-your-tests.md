---
title: "Untangle your Tests"
talk_type: "Lightning Talk"
type: talk
starts_at: 2027-03-17T11:05:00Z
ends_at: 2027-03-17T11:15:00Z
weight: 4
authors:
    - Rudi Svartveit

---
When I write tests, I try to follow these seemingly obvious rules:

1. Tests should break if you don't follow the requirements
2. Tests should pass if you (still) follow the requirements

So, having more tests is always good, right?

Maybe not always. More tests can give you 100% test coverage without a single end-to-end test.
That means that all your endpoints may be broken without a single test case failing, going against rule 1.

Covering implementation details or too many specifics with more tests makes them fragile.
Then the tests no longer function as a safety net when you make changes, breaking rule 2.

In my talk I'll expand upon these points, and wish to leave you with the ability to write fewer tests with more value.
