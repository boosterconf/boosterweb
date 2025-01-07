---
title: "Observability with OpenTelemetry: From Idea to Insight"
talk_type: "Workshop 3t"
type: talk
authors:
    - Hans Kristian Flaatten
    - Øyvind Randa

---
OpenTelemetry might be the biggest thing that has happened in the OpenSource ecosystem since Kubernetes. With a much steeper contribution graph, and the recent General Availability of the specification, OpenTelemetry is now ready for prime time with more and more observability vendors supporting the standard.

Getting insights from your applications is absolutely vital for modern software development teams that want to release new functionality with confidence. The process of programming your application to give this insight is often called instrumentation and we often refer to the three pillars of observability; logs, metrics and tracing.

The instrumentation itself can be a hassle and will take some time depending on the size and of the application and how modern the framework and libraries are, but it is manageable. The real value comes when you instrument enough of the applications within a team or a business domain. 

This also brings us to the challenge – instrumentation technology that works across different languages and frameworks without having to rewrite those applications from scratch. Different vendors have been providing this as their secret sauce if you accept to be cemented into their walled garden and lock this vital information in their monitoring system. Until now with OpenTelemetry! The universal telemetry toolkit for all your observability needs. 

OpenTelemetry is a graduated CNCF project with first release in 2019 after OpenCensus and OpenTracing decided to merge. It has support for all major programming languages (Java, .NET, Go, Python, ++) and more and frameworks such as (Spring ++) has built in support for OpenTelemetry. All major monitoring tools and platforms (such as the Grafana stack) are contributing and supporting OpenTelemetry in one way or another.

In this workshop, you will learn how to get a modern observability stack up and running with the open source monitoring platform from Grafana – the LGTM stack (Loki, Grafana, Temp and Mimir). You will also learn how to instrument different applications with OpenTelemetry SDK and agent, to gain insight into the application’s performance. 
