---
title: "Testing Autonomous Mobile Robots"
talk_type: "Experience report"
type: talk
authors:
    - Mesut Durukal

---
Keywords: Warehouse Automation, Testing Robots, Experience Report
Target Audience: Testers, Developers, Robot Lovers
Teaser: https://youtu.be/WlgaquXdmx0
Slides: https://drive.google.com/file/d/1hZzUQmrs6KNC052HxjH5ZHrfHrJNM7g4/view?usp=sharing

Elevator Pitch
-----------------------------
Warehouse automation is an evolving market with bleeding technology in place. Autonomous mobile robots are used to assist human operators to increase efficiency. How can we test such a system? Let's discuss the challenges and ways to cope with them. 

Problems
-----------------------------
Quality has several aspects such as operability, maintainability, recoverability, performance, usability, and efficiency. These are important for every product, but especially for the Warehouse Automation systems, even more, since the main idea is increasing efficiency and productivity. Recoverability is also very significant since the whole warehouse operations may be stuck in case of a fatal error.

As far as test automation is taken into consideration, there are several challenges coming into place. There are robots and hardware components like sensors and perception modules, making automation more difficult. On top of that, every customer has a different warehouse, meaning a different configuration. The system should be ensured to be supporting different options like the number of robots moving around, the number of items in the warehouse, the map of the area, and the priority of the orders. The reusability of tests should be maintained to reduce duplication and maintenance efforts.

A summarized list of problems is:
* Testability issues
* Hardware dependency
* Non-functional aspects
* Reproduction of bugs
* Continuous maintenance

Solutions
-----------------------------
I will explain how we solved the problem of both automating the challenging test environment and verifying the minimum distance problem. Some of the proposals to cope with the difficulties:
* Performing longevity and endurance tests along with benchmarks
* Test Simulation 
* Evidence Collection and ROS Bag
* Improving Code Quality

Conclusion and Outcomes
-----------------------------
Warehouse Automation and Autonomous Mobile Robots development is a hot topic. In an online shopping and delivery world, big companies are investing in developing these systems. By applying the proposed solutions, quality issues are reduced, and customer satisfaction is ensured.

Takeaways
-----------------------------
* A basic understanding of the warehouse automation systems
* Ways to Ensure the Performance of a Robotics System
* Non-functional aspects of quality in a product with SW, embedded modules, and HW: Chaos testing, Environmental tests
* Ways to improve code quality in the test automation framework

