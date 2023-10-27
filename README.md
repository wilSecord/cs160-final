---

Assignment Name: Algorithms F23 Final Project  
Group Members: Wil Secord, Chloe Halverson, Will Titus, Theo King
Repository: https://github.com/wilSecord/cs160-final  

---
# Midterm Report: U.S. States & Musical Artists

In this project, we will investigate the level of connection between musical artists and every U.S. state.

## System Design

**TODO!**

## Milestone Progress 

In our [initial report](https://github.com/wilSecord/cs160-final/tree/78f0d9aad897440c5330bc14cc4e8165f69dc3ca), we had two distinct goals: finished data gathering and starting to work on analysis.

Since then, we have gathered all of the requisite data. Our source dataset, the MusicBrainz database, has an incredible amount of albums; songs; bands; and even the instruments used -- for our initial analysis, to simplify our representation, we are not using this data. Our difficulty comes from extracting the required data and nothing else -- to do this, we learned the PL/PGSQL procedural language and the Postgres dialect of SQL. 

We had difficulties with GitHub's requirements for file size, so we split some table files (CSVs) into multiple 'chunk's. The procedure of getting all data and splitting was then automated into a shell script. 

We also had some difficulty with query complexity, which was solved through usage of Postgres's query planner and `EXPLAIN` queries. We optimized by removing joins on tables which weren't strictly necessary, which increased our total size significantly (as we weren't able to filter to American artists), but made the query time managable.

We have done some basic analysis on our dataset, which has helped us to further refine our process. For example, we have [removed duplicate links](https://github.com/wilSecord/cs160-final/commit/eeed858a5ebff936cae2f65253626417a2fd90fe) from the dataset by taking advantage of artist IDs' nature as integers.

We were able to acheive our milestones. Although our progress on our analysis goal was not as deep as we could have hoped for, it was able to inform our other goal; when initially framing our milestones, we mainly included this as a more vague secondary goal, so it is acceptable to achieve concrete benefit, even if it's not significant.

We don't forsee that we will have to change our milestones. We are confident in our ability to complete analysis, and we have done investigation on visualizers in the event that we are not able to create our own.

### Implementation Decisions 

We *are* implementing various graph algorithms, but we haven't finished anything concretely yet. Even so, our main architectural choice is a commitment to the use of Python where possible because all of our group members are aware of how to use it. We are planning to use the NetworkX library for management of our graph, as it is well-known in the industry and came well-recommended by our professor. We will reimplement algorithms that we use in order to , with an eye towards NetworkX's implementation for correctness purposes.

We also considered using an external server, such as Blazegraph, or implementing our own graph type. The former was discarded because although an external server provides speed in the SPARQL query language, it limits our flexibility and ease of implementation for external algorithms. The latter was discarded after careful consideration because we want to be easily able to monitor the 

We are concerned about the speed ramifications of Python, but we are prepared to develop in Python and transition to Java if required. We have made the decision to defer consideration of Java graph libraries until this becomes an issue.

