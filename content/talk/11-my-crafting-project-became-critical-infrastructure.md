---
title: "My crafting project became critical infrastructure"
talk_type: "Experience report (30 min)"
authors:
    - Elizabeth Zagroba
---
Driven to madness by the normal workflow for testing my application, I wrote a small Python script in a couple of days. It called some APIs to build the app and deploy it to a hosted environment. It ran in my terminal, printing output often enough that I wouldn't get distracted. It solved my immediate problem. 

But that wasn't the only problem it solved. It replaced a manual piece of our release process with an automated step, allowing my team to automate our pipeline. Then other teams copied us. Soon, a dozen teams in three units were trying to add and request features so that my personal pet project could become part of their merge request and release pipelines too. As more ideas needed to urgently serve the needs of teams in release time crunches, I merged code I didn't agree with in to keep everyone unblocked. The code base became something I dreaded, and I stopped maintaining it. 

The next time a merge request came in, I was able to pay it the time and attention it deserved. I worked *with* the code submitter to improve usability. Another dev forked the code to build a UI component, serving a completely different purpose. Seeing how many individuals and teams used this code reignited my interest in maintaining it. I wrote tests for the repository, allowing me to finally refactor away the changes I'd dreaded. And the next contributor to the code base added a test without being asked. I no longer dread my little Python script. I support and maintain a critical piece of infrastructure, and I'm excited to do it.
