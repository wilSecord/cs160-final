---

Assignment Name: Algorithms F23 Final Project  
Group Members: Wil Secord, Chloe Halverson, Will Titus, Theo King
Repository: https://github.com/wilSecord/cs160-final  

---
# U.S. States' Degree of Influence on Musical Artists

This project is an analysis of the degree of connection from _each U.S. state_ to the musical industry as a whole.

## Installing & Running

To run this project, you must have a [Postgres MusicBrainz database](https://musicbrainz.org/doc/MusicBrainz_Database/Download) replica. All results in this repository were made from a September 2023 database dump. 

You should also have a recent version of **Python** with `networkx`. A recent version of **Rust** is required for hosting interactive visualizations.

### Connecting to your Postgres database.

Scripts in this repo which require a Postgres connection expect a file `data.txt` in the repository root. The `data.txt` file should follow this format:

```
<username>
<host>
<port number>
<dbse>
<pass>
```

Newlines should be in the Unix format.

### Downloading Data Subset 

To download the subset of data which this project uses from your Postgres database, run `re-get-data.sh`.

Note that by default, this will break all data files into chunks that are under `40M`, such that they will be uploadable to GitHub. To adjust this threshold, use `MAX_FILESIZE_BYTES=<threshold> ./re-get-data.sh`

```console
user@computer /path/to/repo/cs160-final $ ./re-get-data.sh 
```

### Running Analysis 

Our analysis runs using the [NetworkX](https://networkx.org) Python library. It only considers links through the network of American artists for whom we know state associations. For example, a Californian artist connected to a Floridian through a Canadian wouldn't be considered.

We chose to do this because aggregating data for all artists takes a prohibitively lengthy amount of time, so focusing on our specific research question means that our analysis is able to finish in a timely fashion.

To run aggregated analysis, run the `graph.py` file in the repository root. This will print a CSV of analysis on a per-state basis.

```console
user@computer /path/to/repo/cs160-final $ python3 graph.py
```

For static image visualizations of this aggregated analysis, please see the [Results](#Results) section below.

### Running Visualization

Our interactive visualization runs using the [Petgraph](https://github.com/petgraph/petgraph) Rust library. It incorporates *all* artists: this is because artists can be selected arbitrarily at runtime, rather than showing aggregated answers. Although this may seem less applicable to our specific research question, user research shows that, in an interactive format, users prefer this. We have elected to instead have the visualization display a warning when a non-American artist is selected.

## Objectives & Non-Objectives

This project is an attempt to:

- Analyze the musical industry as a whole
- Associate musical artists with U.S. states
- Visualize connection to states in the industry, such that
	- Users can understand where an **artist** is inspired by (other than their own home state)
	- Users can understand which **states** inspire the most artists.

This project does **not** attempt to:

- Analyze **musical style**, **genre**, **listening cohorts**, **amount of work**, or any other aspect of musical creation
- Analyze over **time**, **album series number**, **degree of collaboration**, or any other axis.
- Analyze geography outside of the United States

## 

## Results

## Project Retrospective

