# U.S. States' Degree of Influence on Musical Artists

In this project, we will examine the degree of connection between musical artists and U.S. states in order to see which states have the greatest connection.

## Problem Statement 

Besides the intrinsic cultural value of art, artistic pursuits are incredibly economically valuable[^1]. Music is unique among artistic pursuits as it combines the possibility of **small-scale production** (i.e. made by a single person or a small group, as opposed to movies or TV shows) with **a high degree of collaboration** (i.e. as opposed to book publishing, which, outside of the academic sphere, is a mostly solo pursuit[^2]). Musical artists can collaborate with a large amount of other artists, each artist contributing a personal influence to the resulting work.  Musical works also have **enormous public appeal**.  As such, it's important to understand the source of cultural influence on musical style. 

In this project, we have decided to work on this issue because of a shared love of music and a common interest in large data. In addition, the public availability of data on musical artists and releases is a draw, as databases such as [MusicBrainz](https://musicbrainz.org/) are of a size that we can conceivably process, but still large enough to yield interesting conclusions. 

We have elected to work specifically on the issue of connection to U.S. states because it is an unambiguous geographical separation, which is often extended to a cultural separation[^3]. Although states may share cultures with their neighbors, regions are ill-defined and too coarse-grained, while states are more precise.

<<<<<<< HEAD
### Our Project 
=======
In this project, we will be using New Jersey as an example for several reasons:
- Lack of recognized musical industry/style (unlike California[^4], NYC[^4], Chicago[^5], etc)
- Lack of association with a specific genre (unlike Nashville, Texas, etc)
- Large amount of artists[^4]
- Geographic proximity to New York with distinct culture
>>>>>>> 2dcb70f (Note distinct culture)

In a way similar to the Erdos number[^6] or the Bacon number[^7], we will count the separation between each state and each artist. An artist has a separation of 0 degrees from the state of their hometown, having grown up immersed in that state's culture. This forms the base case of our degree of separation. 

In making this dataset, we hope to see the overall degree of connection from the music industry to each state.

**Note:** An artist who grew up in a state will **not** contribute to that state's connection. We are not attempting to determine the *connection*, *collaboration*, and *influence* of a state's culture; we are not determining the relative *artistic gifts* of their population.

### Itemized Goals

- A graph database describing collaborations between all musical artists
- A method to tell the "degree of separation" between a given artist and a given state 
- A set of shortest paths from each artist to each state 
- An interactive visualizer for our results  

### Stretch Goals 

- Analysis of artists from other countries 
- Incorporation of genre, style, age, etc. in data analysis 
- Degrees of collaboration, in order to more accurately determine influence (e.g. discriminating between 'one-off' collaborations and frequent collaborations) 
- Public availability of interactive visualizer 
- Analysis of [ListenBrainz](https://listenbrainz.org/) database for musical similarity of inspired artists  

## Expected Timeline

### Midterm 

### Final 

## Potential Challenges 

- 

## Related Work 

Investigation of the difference between states' culture, without the inter-state influence or musical aspects of our project.

> Rentfrow, Peter. J. “Statewide differences in personality: Toward a psychological geography of the United States” _American Psychologist, 65_(6), 548–558_, psycnet.apa.org/record/2010-17989-002. Accessed 24 Sept. 2023.

Investigation of collaboration in music and definition of types of working relationships. This defines a typology of collaborative music, but does not analyze big data about music.

> Taylor, Alan. ““Collaboration” in Contemporary Music: A Theoretical View.” _Contemporary Music Review_, vol. 35, no. 6, Nov. 2016, pp. 562–578, https://doi.org/10.1080/07494467.2016.1288316. Accessed 24 Sept. 2023.

Conversion of the MusicBrainz database schema into graph database format. This describes the conversion, but makes no attempt to specifically analyze collaborations.

> Jacobson, Kurt, Simon Dixon, and Mark Sandler. "LinkedBrainz: Providing the MusicBrainz next generation schema as linked data." _Late-Breaking Demo Session at the 11th International Society for Music Information Retrieval Conference_. 2010. Accessed 24 Sept. 2023.

Music collaboration investigation, linked to graph theory

> _Graph Theory in Music Artist Collaborations : Networks Course Blog for INFO 2040/CS 2850/Econ 2040/SOC 2090_. blogs.cornell.edu/info2040/2022/09/21/graph-theory-in-music-artist-collaborations/. Accessed 24 Sept. 2023.

Interactive service covering collaboration with focus on derivative works

> Hamasaki, Masahiro, and Masataka Goto. "Songrium: A music browsing assistance service based on visualization of massive open collaboration within music content creation community." _Proceedings of the 9th International Symposium on open collaboration_. 2013.


>


>


>


>


>



[^1]: https://www.arts.gov/impact/research/arts-data-profile-series/adp-28
[^2]: https://www.newyorker.com/books/page-turner/can-you-write-a-novel-as-a-group
[^3]: https://psycnet.apa.org/record/2010-17989-002
[^4]: Wikidata - SPARQL query, "number of notable musicians per U.S. state"; see [`musicians.spaqrl`](./musicians.sparql) for source
[^5]: https://web.archive.org/web/20080513152119/http://members.socket.net/~dcowsley/jazzstyles.htm
[^6]: https://sites.google.com/oakland.edu/grossman/home/the-erdoes-number-project
[^7]: https://archive.org/details/sixdegreesofkevi00fass
