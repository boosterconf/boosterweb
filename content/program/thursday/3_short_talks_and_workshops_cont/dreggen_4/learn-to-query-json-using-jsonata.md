---
title: "Learn to query JSON using JSONata"
talk_type: "Workshop 1,5h"
type: talk
starts_at: 2027-03-18T10:50:00Z
ends_at: 2027-03-18T12:20:00Z
weight: 1
authors:
    - Ove Gram Nipen

---
Let's say you are integrating with a new REST service you don't already know. You call an endpoint in Postman and get a few hundred (or thousands!) lines of JSON in return. Then it's your job to figure out what this all means, and how the result changes when you call the service with different parameters. Or you have a lot of configuration files and you want to compare some of the settings across all of them. 

Scrolling through a wall of JSON is a challenge, especially if you're interested in parts of the content which is many lines apart. What if you could write a query against the JSON document to make it shorter and simpler?

In this workshop you'll learn how to use the query language JSONata to extract parts of a large JSON document and transform and flatten deeply nested structures, so that you can easily understand the content of the document without having to scroll a lot. You'll be writing small queries in an online editor and having the results instantly. 

You'll also learn how to use JSONata in your code, so that when you call other REST services as a client, you can use JSONata to transform the response to fit your own custom DTO classes. This is useful when the response is deeply nested and you are only interested in a small part of it. Using these techniques you can simplify and adapt the structure of the data so that they fit better with your use case in your application. You'll experience how this simplifies your code and makes it easier to test.

The objective of this workshop is to let you try JSONata hands-on, so that you can decide if it's worth pursuing further. 



Outline

1. Introduction by the workshop hosts (30 min)
- Introduction to the problem of navigating and reading large JSON documents
- Overview of the concept of JSON query languages and their use
- Example queries, with explanation of new concepts for each query
- Introduction to the problem of deserializing large and deeply nested JSON documents
- Examples of how to use JSONata in Javascript, Java and .NET

2. Hands-on programming with prepared exercises and host guidance (60 min)
- Tasks that involve the online editor at try.jsonata.org
- Tasks that involve programming with JSONata in .NET, Java or Javascript (whichever you prefer)

