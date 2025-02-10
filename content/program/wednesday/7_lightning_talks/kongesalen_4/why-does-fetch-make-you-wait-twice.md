---
title: "Why does Fetch make you wait twice?"
talk_type: "Lightning talk"
type: talk
weight: 9
authors:
    - Truls Henrik Jakobsen

---
Anyone who's worked with the Fetch API in JavaScript has probably wondered why you have to await twice — first for the response and then for the body. If you're anything like me, you probably just accepted this as how the API works and moved on with your work. But now I've gone back and done a deep dive into Fetch so you don't have to!

We'll take a look at how data is transmitted over the Internet and have a closer look at Streams in Javascript. Expect to gain a better understanding of their funky features and how you can use them to your advantage. At the very least, you'll leave with the satisfaction of actually understanding why you need to wait twice when using Fetch.

