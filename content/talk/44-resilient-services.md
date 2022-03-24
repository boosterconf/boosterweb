---
title: "Resilient services"
talk_type: "Workshop (3 hours)"
authors:
    - Bjørn Einar Bjartnes
time_slot: "Friday: 09:00 - 12:15"
room: "Kongesalen 1"
---
Users' patience with services not working is gradually decreasing as the quality of services online is improving. With current trends of moving services to the cloud and building smaller and network-intensive services, meeting these expectations can be challenging for us developers. We want to be able to build services that we can run confidently despite partial failures and outages.

In this workshop, you will learn how to simulate latency and failures in your web application and how to add strategies to deal with this. We will learn how changing the different parameters change the behavior of our application under load, and what trade-offs we ultimately must make.

We will use k6 for load testing a .NET Core web API. For resiliency strategies, we will use Polly, and we will use Simmy for fault injection. This is red-green testing, but for performance and resiliency. You will learn about service level indicators, service level objectives, and how to formulate and test such requirements.

{{< tweet 1506906119943376901 >}}
