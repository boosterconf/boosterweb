---
title: "How to draw a triangle"
talk_type: "Workshop (3 hours)"
type: talk
authors:
    - Kristoffer Dyrkorn
---
In this workshop we will take a close look at how computers draw geometry - more specifically, triangles - on screen.

Computer graphics is a vast field that contains many techniques, but since triangle drawing is an important building block for many advanced graphical presentations, also in 3D, this is a topic that is well suited as an introduction. 

By using plain JavaScript and the browser as a runtime platform, we will go through the principles and the practice of converting a mathematical representation of a triangle (vertices and edges) into a group of pixels that closely resemble  that shape on screen. The mathematics will not be complicated (the four operations will suffice) but will still make us able to implement a simple software version of the algorithm modern GPUs run in hardware.

We will first focus on getting things right, and note that the devil is in the details, and then, when our triangles on screen behave, we will show an optimisation trick that will give us a tenfold performance boost!

We will round off the workshop with a more detailed look at how modern GPUs draw triangles in practice - ie topics for future explorations. 
