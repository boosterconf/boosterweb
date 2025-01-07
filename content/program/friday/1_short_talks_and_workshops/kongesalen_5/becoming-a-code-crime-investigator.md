---
title: "Becoming a code crime investigator"
talk_type: "Workshop 3t"
type: talk
authors:
    - Ole Eskild Steensen

---
Traditionally, code analysis focuses on one metric: code complexity. With this narrow approach we disregard the treasure trove of behavioural information found in our version control system. 

In this workshop we'll explore tools and concepts from Adam Tornhill's book "Your Code as a Crime Scene", using data-driven analysis to investigate code for clues hidden in our version control systems.
 
After we've demoed and looked at how to do these behavioural analyses on big open source projects like React and Kuberenetes, you will be running similar analyses on your own codebase. The analysis is code agnostic, and is run locally on your own computer.

Some of the analyses we'll be doing includes:
* Identifying and visualizing "hotspots", a.k.a. areas with high rime rates. That is, modules where code are complex and changes are frequent. In other words, prime candidates for your future refactoring efforts.

* Discovering partners in crime. How does two or more modules change together over time? Expose hidden dependencies and couplings in or across repositories and projects, and reveal which seemingly unrelated files has to change after a code edit.  

By the end you should have produced both visualizations and data about your existing codebase that can be used to guide future decisions in whatever project you're currently working on. 

Bring a laptop and one or more projects that uses Git as their version control system. 
