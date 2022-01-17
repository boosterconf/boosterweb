---
title: "Building a search engine backend with Kafka Streams and Connect "
talk_type: "Workshop (3 hours)"
authors:
    - Øyvind Randa
    - Mads Lundeland
---
You are tasked with providing a state of the art search engine for the audiobook store AudioBooks4You.com. Your first task will be to create a basic search functionality by author and title. Throughout this workshop, you will improve the engine step by step with scoring from GoodReads, aggregated user listening events, and at last your own preferences.

In this workshop we are going to use the power of Kafka to integrate data from several data sources with Kafka Streams and use Kafka Connect to transfer data in and out of Kafka.

Technology stack: 

Apache Kafka
Apache Kafka is an open-source distributed event streaming platform used by thousands of companies for high-performance data pipelines, streaming analytics, data integration, and mission-critical applications.

Kafka Connect
Kafka Connect is a tool for scalably and reliably streaming data between Apache Kafka and other data systems. It makes it simple to quickly define connectors that move large data sets into and out of Kafka. Kafka Connect can ingest entire databases into Kafka topics, making the data available for stream processing with low latency.

Kafka Streams
Kafka Streams is a client library for building applications and microservices, where the input and output data are stored in Kafka clusters. It combines the simplicity of writing and deploying standard Java and Scala applications on the client side with the benefits of Kafka's server-side cluster technology.

Elasticsearch
Elasticsearch is a distributed, free and open search and analytics engine for all types of data, including textual, numerical, geospatial, structured, and unstructured.

