use petgraph::{prelude::NodeIndex, stable_graph::IndexType, Directed, Graph};
use std::{
    cmp,
    collections::{BTreeMap, BinaryHeap, HashMap},
    rc::Rc,
};

pub fn dijkstras_algorithm_shortest_path<N, E, Ix: IndexType>(
    graph: &Graph<N, E, Directed, Ix>,
    start: NodeIndex<Ix>,
    goal: NodeIndex<Ix>,
) -> Option<Vec<NodeIndex<Ix>>> {
    dijkstras_algorithm_shortest_path_to_multiple(graph, start, &vec![goal]).remove(&goal)
}

pub fn dijkstras_algorithm_shortest_path_matrix<N, E, Ix: IndexType>(
    graph: &Graph<N, E, Directed, Ix>,
    nodes: &Vec<NodeIndex<Ix>>,
) -> BTreeMap<(NodeIndex<Ix>, NodeIndex<Ix>), Vec<NodeIndex<Ix>>> {
    let mut result = BTreeMap::new();

    for (i, start) in nodes.iter().enumerate() {
        let paths =
            dijkstras_algorithm_shortest_path_to_multiple(graph, start.clone(), &nodes[i + 1..]);

        for (goal, path) in paths.into_iter() {
            result.insert((start.clone(), goal), path);
        }
    }

    return result;
}

pub fn dijkstras_algorithm_shortest_path_to_all<N, E, Ix: IndexType>(
    graph: &Graph<N, E, Directed, Ix>,
    start: NodeIndex<Ix>,
) -> NodePathSystem {
    let mut frontier = BinaryHeap::new();

    //initialize the paths with the start's path
    let mut paths = NodePathSystem::new_with_capacity_and_root(graph.node_count(), start);

    //Use Reverse to make the priority queue a min-heap instead of a max-heap
    frontier.push(cmp::Reverse((0, start)));

    loop {
        let Some(cmp::Reverse((cost, node))) = frontier.pop() else {
            //if we've exhausted the frontier, we've explored the whole graph. Return the path.
            break;
        };

        //we can unwrap because we only reach this point if we got a node from the frontier,
        //and a node only goes into the frontier if it had a reachable path.
        let node_path = paths.get_unwrap(&node).clone();

        //if we've already found some other, better path to this node, don't bother exploring neighbors
        if node_path.cost < cost {
            continue;
        }

        for neighbor in graph.neighbors(node) {
            let arc_length = 1;

            let previously_known_cost = paths
                .get(&neighbor)
                .as_ref()
                .map(|x| x.cost)
                .unwrap_or(usize::MAX);
            let path_to_neighbor_through_node = node_path.with_added(neighbor.index(), arc_length);

            let new_cost = path_to_neighbor_through_node.cost;

            if new_cost < previously_known_cost {
                //if we haven't explored this before, then put it into the frontier
                //if we explored it already, then we can skip re-exploring all of its
                //children by not putting it into the frontier.
                if !paths.contains_key(&neighbor) {
                    frontier.push(cmp::Reverse((new_cost, neighbor)));
                }

                paths.insert(neighbor, path_to_neighbor_through_node);
            }
        }
    }

    return paths;
}

fn dijkstras_algorithm_shortest_path_to_multiple<N, E, Ix: IndexType>(
    graph: &Graph<N, E, Directed, Ix>,
    start: NodeIndex<Ix>,
    goals: &[NodeIndex<Ix>],
) -> BTreeMap<NodeIndex<Ix>, Vec<NodeIndex<Ix>>> {
    let mut frontier = BinaryHeap::new();

    //initialize the paths with the start as a root
    let mut paths =
        NodePathSystem::new_with_capacity_and_root(graph.node_count(), start);

    //Use Reverse to make the priority queue a min-heap instead of a max-heap
    frontier.push(cmp::Reverse((0, start)));

    let mut results = BTreeMap::new();

    loop {
        let Some(cmp::Reverse((cost, node))) = frontier.pop() else {
            //if we've exhausted the frontier, we've explored the whole graph. Return the path.
            return results;
        };

        //if we've already found some other, better path to this node, don't bother exploring neighbors
        if paths.get_unwrap(&node).cost < cost {
            continue;
        }

        //if this is a goal, put its path (the shortest) into the results map
        if goals.contains(&node) {
            results.insert(node, paths.path_as_vec(&node));
            //...and check if we've found all the results (and can early-return)
            if results.len() == goals.len() {
                return results;
            }
        }

        for neighbor in graph.neighbors(node) {
            let arc_length = 1;

            let previously_known_cost = paths.get(&neighbor).map(|x| x.cost).unwrap_or(usize::MAX);
            let path_to_neighbor_through_node =
                paths.get(&node).unwrap().with_added(neighbor.index(), graph.node_weight(neighbor), arc_length);

            let new_cost = path_to_neighbor_through_node.cost;

            if new_cost < previously_known_cost {
                frontier.push(cmp::Reverse((new_cost, neighbor)));

                paths.insert(neighbor, path_to_neighbor_through_node);
            }
        }
    }
}

#[derive(Clone)]
struct NodePath {
    self_id: usize,
    cost: usize,
    to: Option<usize>,
    size: usize,
}

#[derive(Clone)]
pub struct NodePathSystem(Vec<Option<NodePath>>);

impl NodePathSystem {
    pub fn new_with_capacity(capacity: usize) -> Self {
        NodePathSystem(vec![None; capacity])
    }
    pub fn new_with_capacity_and_root<N: IndexType>(capacity: usize, key: N, key_id: usize) -> Self {
        let mut inner = vec![None; capacity];

        inner[key.index()] = Some(NodePath {
            self_id: key_id,
            cost: 0,
            size: 1,
            to: None,
        });

        NodePathSystem(inner)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn insert_link(&mut self, from_index: usize, from: usize, to: usize) {
        let inner = &mut self.0;
        inner[from_index] = Some(NodePath {
            self_id: from,
            cost: 0,
            to: if to == from { None } else { Some(to) },
            size: 1,
        });
    }
    fn insert<N: IndexType>(&mut self, key: N, value: NodePath) {
        self.0[key.index()] = Some(value);
    }
    fn get<N: IndexType>(&self, key: &N) -> Option<&NodePath> {
        self.0.get(key.index())?.as_ref()
    }
    fn get_unwrap<N: IndexType>(&self, key: &N) -> &NodePath {
        self.0.get(key.index()).unwrap().as_ref().unwrap()
    }

    pub fn path_as_vec<N: IndexType>(&self, node: &N) -> Vec<N> {
        let mut result =
            Vec::with_capacity(self.get(node).as_ref().map(|x| x.size).unwrap_or_default());

        let mut node = node.index();

        let vals = &self.0;

        loop {
            result.push(N::new(node));

            node = match vals[node] {
                Some(NodePath {
                    to: Some(ref n), ..
                }) => {
                    if n == &node.index() {
                        break;
                    } else {
                        *n
                    }
                }
                _ => break,
            }
        }

        result
    }

    fn contains_key<N: IndexType>(&self, key: &N) -> bool {
        let key = key.index();

        key < self.0.len() && self.0[key].is_some()
    }

    fn into_iter<N: IndexType>(self) -> impl Iterator<Item = (N, usize)> {
        self.0
            .into_iter()
            .flatten()
            .enumerate()
            .filter_map(|(k, v)| Some((N::new(k), v.to?)))
    }

    pub fn iter<'a, N: IndexType>(&'a self) -> impl Iterator<Item = (N, &'a usize)> {
        self.0
            .iter()
            .flatten()
            .enumerate()
            .filter_map(|(k, v)| match v.to {
                Some(ref v) => Some((N::new(k), v)),
                _ => None,
            })
    }

    pub fn path_length_calc<N: IndexType>(&self, node: &N) -> usize {
        let mut node = node.index();
        let mut result = 0;

        loop {
            result += 1;

            node = match self.0[node] {
                Some(NodePath {
                    to: Some(ref n), ..
                }) => {
                    let n = n.index();
                    if n == node {
                        break;
                    }
                    n
                }
                _ => break,
            }
        }

        result
    }
}

impl PartialEq for NodePath {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for NodePath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Eq for NodePath {}

impl Ord for NodePath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl NodePath {
    pub fn with_added(&self, head: usize, head_id: usize, length: usize) -> Self {
        NodePath {
            self_id: head_id,
            to: Some(head),
            cost: self.cost + length,
            size: self.size + 1,
        }
    }
    pub fn size(&self) -> usize {
        self.size
    }
}
