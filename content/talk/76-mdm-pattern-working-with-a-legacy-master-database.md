---
title: "MDM Pattern - working with a legacy master database "
talk_type: "Lightning talk"
authors:
    - Stig Nielsen
---
The background:
Your master data is a legacy database - it contains "everything" your business depend upon. You do want to create new solutions to support your business's everyday operations but querying the legacy database is slow and error prone. What to do? 
 
The talk:
We have developed a M(aster)D(ata)M(anagement) pattern, extracting what we need and storing it using an up-to-date data model which fits well for whatever new use case we have. 
 
We do this using a neat selection of managed services in Azure. The outcome is no latency anymore, and still having fresh data in our new applications.  
 
I will show the diagrams, explain the pitfalls and the solutions - and hopefully spark a few ideas for others with similar use cases in their organizations.

Keywords: T-SQL RowVersion. Azure Functions. Anti corruption layer. 
 
Disclaimer: I am a backend programmer / solution architect. I suspect a DB expert would approach this problem differently using other tools.

