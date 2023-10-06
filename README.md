---

Assignment Name: Algorithms F23 Final Project  
Group Members: Wil Secord, Chloe Halverson, Will Titus  
Repository: https://github.com/wilSecord/cs160-final  

---
# U.S. States' Degree of Influence on Musical Artists

In this project, we will examine the degree of connection between musical artists and U.S. states in order to see which states have the greatest connection.

## Problem Statement 

Besides the intrinsic cultural value of art, artistic pursuits are incredibly economically valuable[^1]. Music is unique among artistic pursuits as it combines the possibility of **small-scale production** (i.e. made by a single person or a small group, as opposed to movies or TV shows) with **a high degree of collaboration** (i.e. as opposed to book publishing, which, outside of the academic sphere, is a mostly solo pursuit[^2]). Musical artists can collaborate with a large amount of other artists, each artist contributing a personal influence to the resulting work.  Musical works also have **enormous public appeal**.  As such, it's important to understand the source of cultural influence on musical style. 

In this project, we have decided to work on this issue because of a shared love of music and a common interest in large data. In addition, the public availability of data on musical artists and releases is a draw, as databases such as [MusicBrainz](https://musicbrainz.org/) are of a size that we can conceivably process, but still large enough to yield interesting conclusions. 

We have elected to work specifically on the issue of connection to U.S. states because it is an unambiguous geographical separation, which is often extended to a cultural separation[^3]. Although states may share cultures with their neighbors, regions are ill-defined and too coarse-grained, while states are more precise.

### Our Project 

In a way similar to the Erdos number[^6] or the Bacon number[^7], we will count the separation between each state and each artist. An artist has a separation of 0 degrees from the state of their hometown, having grown up immersed in that state's culture. This forms the base case of our degree of separation. 

In making this dataset, we hope to see the overall degree of connection from the music industry to each state.

**Note:** An artist who grew up in a state will **not** contribute to that state's connection. We are not attempting to determine the *connection*, *collaboration*, and *influence* of a state's culture; we are not determining the relative *artistic gifts* of their population.

### Itemized Goals

- A method to tell the "degree of separation" between a given artist and a given state
- Analysis of the graph formed by the degrees of separation and connection
  - Specifically, shortest-path; graph connectedness; etc
  - However, we will attempt to implement multiple algorithms in order to gain an accurate view of the music industry's overall connection
- An interactive visualizer for our results

### Stretch Goals 

- Analysis of artists from other countries 
- Incorporation of genre, style, age, etc. in data analysis 
- Degrees of collaboration, in order to more accurately determine influence (e.g. discriminating between 'one-off' collaborations and frequent collaborations) 
- Public availability of interactive visualizer 
- Analysis of [ListenBrainz](https://listenbrainz.org/) database for musical similarity of inspired artists

## Expected Timeline

### Midterm 

- **Graph representation of collaborations**
- Begin work on analysis of graph representation 
- Begin work on interactive visualizer of experimental subset of the data 

### Final 

- Full analysis results
- Finished interactive visualizer 

## Potential Challenges 

- Dataset size: we may anticipate difficulties in loading and transforming the complete dataset, and then visualizing our data interactively 
- Runtime: the complete creation of our graph database may take a prohibitive amount of time 

## Related Work 

Investigation of the difference between states' culture, without the inter-state influence or musical aspects of our project.

> Rentfrow, Peter. J. “Statewide differences in personality: Toward a psychological geography of the United States” _American Psychologist, 65_(6), 548–558_, psycnet.apa.org/record/2010-17989-002. Accessed 24 Sept. 2023.

Investigation of collaboration in music and definition of types of working relationships. This defines a typology of collaborative music, but does not analyze big data about music.

> Taylor, Alan. ““Collaboration” in Contemporary Music: A Theoretical View.” _Contemporary Music Review_, vol. 35, no. 6, Nov. 2016, pp. 562–578, https://doi.org/10.1080/07494467.2016.1288316. Accessed 24 Sept. 2023.

Conversion of the MusicBrainz database schema into graph database format. This describes the conversion, but makes no attempt to specifically analyze collaborations.

> Jacobson, Kurt, Simon Dixon, and Mark Sandler. "LinkedBrainz: Providing the MusicBrainz next generation schema as linked data." _Late-Breaking Demo Session at the 11th International Society for Music Information Retrieval Conference_. 2010. Accessed 24 Sept. 2023.

Music collaboration investigation, linked to graph theory. This talks about why musical collaborations are often popular and about social graphs in fanhood, but doesn't analyze any data about collaborations themselves.

> Xu, Ruqing.  "Graph Theory in Music Artist Collaborations." _Networks Course Blog for INFO 2040/CS 2850/Econ 2040/SOC 2090_. blogs.cornell.edu/info2040/2022/09/21/graph-theory-in-music-artist-collaborations/. Accessed 24 Sept. 2023.

Interactive service covering collaboration with focus on derivative works. Unlike our project, this focuses on a video-sharing service rather than published music.

> Hamasaki, Masahiro, and Masataka Goto. "Songrium: A music browsing assistance service based on visualization of massive open collaboration within music content creation community." _Proceedings of the 9th International Symposium on open collaboration_. 2013.


Investigation of connection in another network (the Web). The findings here make us confident that our data will be connected enough to yield interesting results.

> Stromberg, Joseph. “Any Two Pages on the Web Are Connected by 19 Clicks or Less.” _Smithsonian Magazine_, www.smithsonianmag.com/science-nature/any-two-pages-on-the-web-are-connected-by-19-clicks-or-less-19517004/. Accessed 25 Sept. 2023.

Investigation of search for a specific endpoint in social connection. This is an investigation of a graph which *cannot* be completely known (the global social graph), while we are investigating a well-defined and mostly known graph (musical collaborations). However, we will cover similar topics (degree and method of connection to specifically chosen nodes)

> Dodds, Peter Sheridan, et al. “An Experimental Study of Search in Global Social Networks.” _Science_, vol. 301, no. 5634, 8 Aug. 2003, pp. 827–829, https://doi.org/10.1126/science.1081058.

Investigation of collaboration in the music industry. This considers a subset of albums from authoritative 'greatest albums' lists, not a wide-ranging database. This also lacks our geographic/cultural component, but is a good look at a smaller scale of what we aim to do.

> Budner, Pascal, and Joern Grahl. “Collaboration Networks in the Music Industry.” ArXiv:1611.00377 [Physics], 1 Nov. 2016, arxiv.org/abs/1611.00377.

This is an investigation of wide-ranging collaboration in the music industry. This is global, while our project will be only focusing on the U.S, but we can use some of the same measures as sources to inform our data analysis.

> Topirceanu, Alexandru, et al. “MuSeNet: Collaboration in the Music Artists Industry.” IEEE Xplore, ieeexplore.ieee.org/abstract/document/6984896/authors#authors. Accessed 4 Oct. 2023.

This contains an investigation into collaboration networks in two specific genres, while ours is an investigation into music artists in general. This also uses Discogs as a data source, while we will use MusicBrainz. However, we can use their approach to inform ours in analysis.

> Gienapp, Lukas, et al. "Topological properties of music collaboration networks: The case of Jazz and Hip Hop." _Digital Humanities Quarterly_, 2021, Vol. 15 Issue 1, pN.PAG-N.PAG. 1p



[^1]: https://www.arts.gov/impact/research/arts-data-profile-series/adp-28
[^2]: https://www.newyorker.com/books/page-turner/can-you-write-a-novel-as-a-group
[^3]: https://psycnet.apa.org/record/2010-17989-002
[^4]: Wikidata - SPARQL query, "number of notable musicians per U.S. state"; see [`musicians.spaqrl`](./musicians.sparql) for source
[^5]: https://web.archive.org/web/20080513152119/http://members.socket.net/~dcowsley/jazzstyles.htm
[^6]: https://sites.google.com/oakland.edu/grossman/home/the-erdoes-number-project
[^7]: https://archive.org/details/sixdegreesofkevi00fass
