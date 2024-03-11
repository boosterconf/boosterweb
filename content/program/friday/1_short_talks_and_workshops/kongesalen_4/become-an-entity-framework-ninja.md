---
title: "Become an Entity Framework Ninja 🥷"
talk_type: "Workshop 1,5t"
type: talk
authors:
    - Stig Nielsen
    - Eugene Rebedailo

---
This workshop will consist of a theory part before the practical bit/workshop.

Prerequisites

Participant knowledge:

Basic knowledge of SQL and Entity Framework is useful. 

A notion of what DDD is.

When it comes to setting running the code / template, this is what you need:

* a GitHub account, or else using the template above will prove difficult 💥
* .NET SDK (https://dotnet.microsoft.com/en-us/download) - this is the crux of our exercise. Make sure you at least have this installed.
* IDE for C# (We will be using Visual Studio 2022, with latest updates). In a standard setup, you will then already have EF version 8.x installed. Alternatives:
    - JetBrains Rider
    - VS Code + dotnet CLI
* MS SQL SERVER - Developer - https://www.microsoft.com/en-us/sql-server/sql-server-downloads - there are alternatives, but this workshop is designed for MS SQL Server, so try and stick with it.
* MS SQL Server - Management Studio - https://learn.microsoft.com/en-us/sql/ssms/download-sql-server-management-studio-ssms?view=sql-server-ver16 or another SQL Management tool. Some alternatives:
    - Azure Data Studio - https://learn.microsoft.com/en-us/azure-data-studio/download-azure-data-studio. Great minimal tool, but doesn't allow for a lot of advanced automations or diagram generation.
    - DBeaver - https://dbeaver.io/download/ - Open source and works with a lot of database engines, not just MS SQL.
    - Visual Studio or JetBrains Rider have builtin tools. VS Code extensions also can be used for this as well, but are not as well supported.
* EF core Tools - https://learn.microsoft.com/en-us/ef/core/cli/dotnet#installing-the-tools - those should be installed as a part of Visual Studio, but make sure you can run `dotnet ef` or `dotnet-ef` 

Maximum preparedness checklist:
- You can navigate into the solution folder and run `dotnet build` - this means you have setup .NET correctly
- You can connect to sql local DB via your database management tool of choice
- `dotnet ef` (or `dotnet-ef`) runs successfully. This means you installed the EntityFramework tools correctly.

Github template url: https://github.com/InsuranceTechnologySolutions/boosterconf-ef-ninja

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


