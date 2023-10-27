---

Assignment Name: Algorithms F23 Final Project  
Group Members: Wil Secord, Chloe Halverson, Will Titus, Theo King
Repository: https://github.com/wilSecord/cs160-final  

---
# Midterm Report: U.S. States & Musical Artists

In this project, we are investigating the level of connection between musical artists and every U.S. state.

## System Design

### Overview 

Our system is a collection of scripts written in various languages to download data about music and then perform analysis on it. Each script forms a subcomponent of our complete system. 

We chose to split our system up this way so that we don't have to run the entire pipeline in a single step: although it's easy to string together commands in shell scripts to *compose* components, it's more difficult to break down a pipeline that wasn't written with this in mind.

### Architecture 

#### Database 

We use a clone of the MusicBrainz database made from a 2023-09-xx dump as our source data. This uses a Postgres database: for the setup, we followed MusicBrainz's recommendations. This is the only component that does not communicate with the others through flat file input/outputs.

#### Data Subsetting

We don't need all of the MusicBrainz data, and we want to perform local analysis on the data we *do* need. PGSQL scripts download the subsets of data we need and write it to CSV files, which we then check in to revision control.

#### Combination & Filtering

Doing some calculations on our data, even ones before our basic analysis, is prohibitively expensive on the server because we would need to compare between different data subsets that we download seperately. As such, we are doing some cross-dataset comparison after downloading in order to make sure that we only keep relevant records. We will remove records that are related to international artists, because our scope is limited to America.

#### Analysis 

After forming our dataset, we will analyze it. This step in the pipeline is composed of several discrete scripts; each script may be run at the operator's discretion. We hope that this flexible structure will yield fruitful explorative results that we can use to build a concrete pipeline for output.

#### Output and Visualization

When we have analytical results, we will visualize them. We have evaluated several graph visualization tools: graphvis, mermaid, and the idea of developing a custom visualizer. We have chosen to use graphvis because it will take less time to develop, and is superior to mermaid in ability to handle larger graphs. We are still open to the idea of making a custom visualizer, but this is a stretch goal.

### Component Interaction

Our system can be partitioned along the border of human interaction. Each component, when ran by an operator, writes its output to disk so that later on, a subsequent component may take the data as input. We chose to do this because it facillitates intuitive caching by checking the output files into source control and because it gives us flexibility to combine different languages (e.g. PGSQL and Python). We considered using JSON files, but ultimately decided not to because of the size of our dataset: at some steps, Python would choke on large JSONs.

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

