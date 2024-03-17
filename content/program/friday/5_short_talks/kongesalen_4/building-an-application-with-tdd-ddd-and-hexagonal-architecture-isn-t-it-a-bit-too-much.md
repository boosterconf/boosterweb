---
title: "Building an Application with TDD, DDD and Hexagonal Architecture - Isn't it a bit too much?"
talk_type: "Experience report"
type: talk
recording_url: https://vimeo.com/924293338
authors:
    - Mufrid Krilic

---
Many experienced developers are longing for an opportunity to build an application from scratch. Compared to the heavy legacy applications that you might be dealing with on a daily basis, which are seen as filled with technical debt and incomprehensible architecture and design choices, a fresh start promises that you can finally write code and build an application in "the right way."

This was exactly the opportunity my colleague and I recently got and with high motivation we applied many of the principles that we highly value, such as Test-Driven Development and Domain-Driven Design. On the other hand, experience has taught us that complexity in the code creeps in regardless. We wondered then if the very principles we based ourselves on, could actually contribute to making the code harder to read and maintain in the long run?

This presentation will show how we built a backend application for integration between three different systems based on TDD, DDD and hexagonal architecture. Along the way, we have assessed the complexity of the solution and asked ourselves some questions:
- Have our decisions about isolation of the domain model and abstraction layer between the systems led to less or greater complexity?
- What constitutes a domain model?
- What about deliberately reduced test coverage on selected parts of the code?

The presentation will conclude with a balanced look at the decision-making process we used while we were writing code and whether it was worth applying "best practices" to this application at all.
