---
title: "Optimizing Land-Based Fish Feeding with Node-RED"
talk_type: "Experience report"
type: talk
starts_at: 2027-03-19T11:20:00Z
ends_at: 2027-03-19T11:50:00Z
weight: 2
authors:
    - Øivind Heggland

---
This isn’t just another IoT demo—it’s a real, deployed, and continuously evolving industrial system that feeds thousands of fish every day using Node-RED.

In this experience report, I will share how Aquateknikk transformed a fragile, PLC-dependent setup into a resilient, containerized IIoT platform for land-based fish farming. Today, this system operates across more than 25 locations, utilizing 10 to 40 Raspberry Pi edge devices per site to calculate and execute critical feeding schedules.

We will dive into the technical realities of this transformation. You will see how Raspberry Pis function as crucial edge compute nodes, how we bridged the gap to industrial hardware via Modbus and Moxa I/O modules, and how we architected a CI/CD pipeline capable of supporting real-world, physical deployments at scale using MQTT.

I will also share the messy reality of industrial IoT, including the current challenges and architectural obstacles we are still trying to solve.

Whether you’re building smart factories, automating greenhouses, or just want to see Node-RED operating at scale, this talk will inspire, inform, and maybe even make you laugh (and yes, we do use a “frontend for backend and backend for frontend” architecture).
