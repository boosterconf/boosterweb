---
title: "From bricks to circles: learn the onion architecture"
talk_type: "Workshop (3 hours)"
type: talk
authors:
    - Lars Lønne
---
The layered architecture, with the database on the bottom, is widely used in software development today. I always find testing difficult with this architecture, with all the mocks and stubbing which is necessary to make the system function. Surely, there’s a better way to do it?

As it turns out, there is. In this workshop, we will be exploring the onion architecture. Learn how to isolate the difficult parts, such as database connections and API clients, and make most of your application easily testable with simple, fast unit tests. We will start with an application written in a layered style, and step by step refactor it to the onion architecture. On the way we will go from a few complicated tests, to a large suite of simple and fast tests that covers almost our entire application. 

## Prerequisites

* A laptop and a browser