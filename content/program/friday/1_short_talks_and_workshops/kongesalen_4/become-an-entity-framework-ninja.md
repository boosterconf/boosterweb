---
title: "Become an Entity Framework Ninja 🥷"
talk_type: "Workshop 1,5t"
type: talk
authors:
    - Stig Nielsen
    - Morten Cools

---
This workshop will consist of a theory part before the practical bit/workshop.

Prerequisites

Participant knowledge:

Basic knowledge of SQL and Entity Framework is useful. 

A notion of what DDD is.

Computer requirements:

Rider/Visual Studio/Other dotnet-capable IDE

dotnet 8.0 installed

SQL Server (Express) installed. If using other RDBMS, support might be limited.

Github account for creating repo based on template

Background:

You've been a good architect and done your modelling properly. You have your type hierarchies, your different bounded contexts and so on. Your class diagrams are impeccable. Now all you have to do is to design the schemas accordingly. That is hard! By using examples from a relational database and dotnet entity framework, we will show you tips and tricks on how to achieve this without too many trade-offs.

Theory part:

D(omain) D(riven) D(esign) basics. How do we want to model the world around using common DDD principles? This will be a quick introduction, as it this is not the core topic of the workshop. The examples will be from a domain the presenters know well, which is insurance. An example: Customers have different types of Objects with Covers, and sometimes there are Claims associated with these.

Workshop:

See separate paragraph about prerequisites. We will have a repository ready for the participants to use (Github template). This will be a REST API which already contains the DDD models, but will all storage parts missing. That is what we will create together; a separate Database project.

The goal is to create the schemas required for this sample domain model.

We will touch upon several sub-topics:

How to define the relationships between entities through attributes.

How to utilize the ModelBuilder to add constraints, single column and composite indexes ++.The ModelBuilder contains a lot of features, we will go through quite a few of them.

Type inheritance. Explain the different techniques, TPH, TPT and TPC. In the workshop we will use T(ype) P(er) H(iearchy).

Utilize different schema names (table prefixes). Using one DBContext to rule them all or create separate ones? We will talk about the options, and show at least one example.

Go through the most common EF core commands (we ❤️ using the terminal).

Create migrations

Remove migrations

Apply the migrations

++

The end result should be a functioning API where we can do CRUD operations on a well-defined set of REST resources.
