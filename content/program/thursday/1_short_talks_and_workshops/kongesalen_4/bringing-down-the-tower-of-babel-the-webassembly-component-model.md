---
title: "Bringing down the tower of Babel! The WebAssembly component model."
talk_type: "Workshop 1,5t"
type: talk
authors:
    - Syver Storm-Furru
    - Amar Topalovic

---
One of the great promises of WebAssembly is that you can code in whatever language you want and run it anywhere, including in the browser. Until recently, this hasn't really been the case since you still had to rely on the libraries of your chosen language, which may or may not be non-existent. After all, is your language _really_ turing complete if it does not have a robust 3rd party library for datetime handling? However, WebAssembly is getting closer and closer to fulfilling its original great promise with the advent of the WebAssembly component model.

In this hands-on workshop, participants will learn how the WebAssembly component model works and how it can be used to combine different programming languages into a single artifact that can be hosted anywhere. We will implement WebAssembly components in both TypeScript and Rust, and combine them into an application that will run in the browser as a client-side library, and as a serverless function.

