---
title: "Master https in your application (in Java)"
talk_type: "Workshop (1.5 hours)"
authors:
    - Johannes Brodwall
time_slot: "Thursday: 09:00 - 10:30"
room: "Bugaarden"
---
So, I decided to make my own Certificate Authority. Kidding. Not kidding.

HTTPS and especially client certificates holds the promise to zero-trust architectures of the future and gives a good way to harden your internal communication paths.

Have you ever gotten the feared error message "PKIX Path Building Failed" or just the simple and unhelpful "SSL error" in your browser?

You may have learned about and thought you understood the theory of public-private key encryption. Even so, setting up a server to demand client certificates, issuing certificates and making sure each part knows to trust each other is tricky business.

In this workshop we will explore the necessary code in Java to create certificates, start an app server with https instead of http, making the client trust a self-signed server certificate and make the server request a certificate from the client.

Along the way, you will learn about trust stores, key stores, SSL context and what the dreaded PKIX path validation failure means.

Bring a computer with the Java runtime pre-installed. There will be hands-on exercises. 
