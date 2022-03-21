---
title: "Building a search engine backend with Kafka Streams and Connect "
talk_type: "Workshop (3 hours)"
authors:
    - Øyvind Randa
    - Mads Lundeland
---
You are tasked with providing a state-of-the-art search engine for the audiobook store AudioBooks4You.com.
Your first task will be to create basic search functionality, but by solving exercises you will improve
the engine step by step.

In this workshop you will learn how to use Kafka Connect with Debezium to stream data from a relational database
to Kafka, use Kafka Streams to transform, join and aggregate data, and another Kafka Connect plugin to stream
results to Elasticsearch.

NB! We highly recommend you to go through the [Getting Started](https://github.com/mlundela/kafka-streams-workshop#getting-started)
section before attending this workshop. By doing so, you will be prepared to start working on the fun parts
straight away.

### Requirements

- JDK 17
- Maven
- Docker Compose

### Technology stack

**Apache Kafka** is an open-source distributed event streaming platform used by thousands of companies for high-performance data pipelines, streaming analytics, data integration, and mission-critical applications.

**Kafka Connect** is a tool for scalably and reliably streaming data between Apache Kafka and other data systems. It makes it simple to quickly define connectors that move large data sets into and out of Kafka. Kafka Connect can ingest entire databases into Kafka topics, making the data available for stream processing with low latency.

**Kafka Streams** is a client library for building applications and microservices, where the input and output data are stored in Kafka clusters. It combines the simplicity of writing and deploying standard Java and Scala applications on the client side with the benefits of Kafka's server-side cluster technology.

**Elasticsearch** is a distributed, free and open search and analytics engine for all types of data, including textual, numerical, geospatial, structured, and unstructured.

