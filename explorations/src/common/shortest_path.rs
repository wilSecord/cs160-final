use petgraph::{prelude::NodeIndex, stable_graph::IndexType, Graph, Directed};
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
) -> HashMap<NodeIndex<Ix>, NodeIndex<Ix>>{
    let mut frontier = BinaryHeap::new();
    let mut paths: Vec<Option<NodePath<NodeIndex<Ix>>>> = vec![None; graph.node_count()];
    paths.fill_with(|| None);

    //Use Reverse to make the priority queue a min-heap instead of a max-heap
    frontier.push(cmp::Reverse((0, start)));

    //initialize the paths with the start's path
    paths[start.index()] = Some(NodePath::new(start));

    loop {
        let Some(cmp::Reverse((cost, node))) = frontier.pop() else {
            //if we've exhausted the frontier, we've explored the whole graph. Return the path.
            break;
        };

        //if we've already found some other, better path to this node, don't bother exploring neighbors
        if paths[node.index()].as_ref().unwrap().cost < cost {
            continue;
        }

        for neighbor in graph.neighbors(node) {
            let arc_length = 1;

            let previously_known_cost = paths[neighbor.index()].as_ref().map(|x| x.cost).unwrap_or(usize::MAX);
            let path_to_neighbor_through_node =
                paths[node.index()].as_ref().unwrap().with_added(neighbor, arc_length);

            let new_cost = path_to_neighbor_through_node.cost;

            if new_cost < previously_known_cost {
                frontier.push(cmp::Reverse((new_cost, neighbor)));

                paths[neighbor.index()] = Some(path_to_neighbor_through_node);
            }
        }
    }

    return paths
        .into_iter()
        .enumerate()
        .filter_map(|(k, v)| Some((NodeIndex::<_>::new(k), NodeIndex::new(v?.prev?.head.index()))))
        .collect();
}

fn dijkstras_algorithm_shortest_path_to_multiple<N, E, Ix: IndexType>(
    graph: &Graph<N, E, Directed, Ix>,
    start: NodeIndex<Ix>,
    goals: &[NodeIndex<Ix>],
) -> BTreeMap<NodeIndex<Ix>, Vec<NodeIndex<Ix>>> {
    let mut frontier = BinaryHeap::new();
    let mut paths = BTreeMap::<(NodeIndex<Ix>, NodeIndex<Ix>), NodePath<NodeIndex<Ix>>>::new();

    //Use Reverse to make the priority queue a min-heap instead of a max-heap
    frontier.push(cmp::Reverse((0, start)));

    //initialize the paths with the start's path
    paths.insert((start, start), NodePath::new(start));

    let mut results = BTreeMap::new();

    loop {
        let Some(cmp::Reverse((cost, node))) = frontier.pop() else {
            //if we've exhausted the frontier, we've explored the whole graph. Return the path.
            return results;
        };

        //if we've already found some other, better path to this node, don't bother exploring neighbors
        if paths[&(start, node)].cost < cost {
            continue;
        }

        //if this is a goal, put its path (the shortest) into the results map
        if goals.contains(&node) {
            println!("Found path from {:?} to {:?}", start, node);
            results.insert(node, paths[&(start, node)].clone().into());
            //...and check if we've found all the results (and can early-return)
            if results.len() == goals.len() {
                return results;
            }
        }

        for neighbor in graph.neighbors(node) {
            let arc_length = 1;

            let previously_known_cost = paths
                .get(&(start, neighbor))
                .map(|x| x.cost)
                .unwrap_or(usize::MAX);
            let path_to_neighbor_through_node = paths
                .get(&(start, node))
                .unwrap()
                .with_added(neighbor, arc_length);

            let new_cost = path_to_neighbor_through_node.cost;

            if new_cost < previously_known_cost {
                frontier.push(cmp::Reverse((new_cost, neighbor)));

                paths.insert((start, neighbor), path_to_neighbor_through_node);
            }
        }
    }
}

#[derive(Clone)]
pub struct NodePath<N: Clone> {
    cost: usize,
    prev: Option<Rc<NodePath<N>>>,
    head: N,
    size: usize,
}

impl<Ix: IndexType> PartialEq for NodePath<Ix> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<Ix: IndexType> PartialOrd for NodePath<Ix> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl<Ix: IndexType> Eq for NodePath<Ix> {}

impl<Ix: IndexType> Ord for NodePath<Ix> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl<N: Clone> NodePath<N> {
    pub fn new(head: N) -> Self {
        NodePath {
            prev: None,
            head,
            cost: 0,
            size: 1,
        }
    }
    pub fn with_added(&self, head: N, length: usize) -> Self {
        NodePath {
            prev: Some(Rc::new(self.clone())),
            head,
            cost: self.cost + length,
            size: self.size + 1,
        }
    }
    pub fn size(&self) -> usize {
        self.size
    }
}

impl<N: Clone> From<NodePath<N>> for Vec<N> {
    fn from(value: NodePath<N>) -> Self {
        let mut vec = Vec::with_capacity(value.size);

        let mut value = &value;
        loop {
            vec.insert(0, value.head.clone());

            let Some(ref v) = value.prev else {
                return vec;
            };
            value = Rc::as_ref(v);
        }
    }
}
