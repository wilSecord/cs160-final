import csv
import networkx as nx
import matplotlib.pyplot as plt
import pickle
import os

# this feels really innefficient but idk how to make it better
def make_a_s_csv():
    a_ids = open('data/artist_ids.csv')
    s_ids = open('data/state_ids.csv')
    a_s = open('data/artist_states.csv')
    n_rels = open('data/artist_states_names.csv', 'w+')
    
    a_reader = csv.reader(a_ids)
    s_reader = csv.reader(s_ids)
    a_s_reader = csv.reader(a_s)
    out = csv.writer(n_rels)
    
    states = {}
    artists = {}
    
    for item in a_reader:
        artists[item[0]] = item[1]
    
    for item in s_reader:
        states[item[0]] = item[1]
    
    
    i = 0
    for item in a_s_reader:
        if i > 0:
            out.writerow([artists[item[0]], states[item[1]]])
        i += 1

def bfs(source: str, graph: nx.Graph):
    frontier = [(0, source)]
    paths = { source: (0, None) } 

    while len(frontier) > 0:
        (node_cost, node) = frontier.pop()

        if paths[node][0] < node_cost:
            continue

        for neighbor in graph.neighbors(node):
            neighbor_path = paths.get(neighbor, None)
            neighbor_cost = node_cost + 1

            if neighbor_path == None or neighbor_cost < neighbor_path[0]:
                if neighbor_path == None:
                    frontier.append((neighbor_cost, neighbor))

                paths[neighbor] = (neighbor_cost, node)

    return paths 

def bfs_each_state(g):
    states = [x[1] for x in csv.reader(open('data/state_names.csv'))]

    bfs_trees = dict()

    for state in states:
        bfs_trees[state] = bfs(state, g)
    return bfs_trees

def print_bfs_tree_stats(bfs_trees: dict[str, dict[str, tuple[int, str]]]):

    with open("./results/num_connected.txt", 'w') as file:

        print("state,artist_count,avg_degree,max_degree,weighted_linear_degree_sum,weighted_quad_degree_sum",file=file)

        for state, tree in bfs_trees.items():
            lengths = [x for x in map(lambda x: x[0], tree.values())]

            avg = 0
            count = 0
            max_val = 0
            weighted_linear = 0.0
            weighted_sum_quad = 0.0

            for i,x in enumerate(lengths):
                if x == 0: continue

                count += 1
                avg = (avg * i + x) / (i + 1)
                max_val = max(max_val, x);

                weighted_linear += 1.0 / (x)
                weighted_sum_quad += 1.0 / (x ** 2)

            if(count): weighted_linear /= count
            if(count): weighted_sum_quad /= count

            print(f"{state},{count},{avg},{max_val},{weighted_linear},{weighted_sum_quad}", file=file)

def make_collab_graph():
    g = nx.Graph()
    
    n_rels = open('data/artist_states_names.csv')
    n_collab = open('data/collaborations_names.csv')
    n_rel_reader = csv.reader(n_rels)
    n_col_reader = csv.reader(n_collab)
    n_states = csv.reader(open('data/state_names.csv'))
    for item in n_states:
        g.add_node(item[1])

    for item in n_rel_reader:
        g.add_node(item[0])
        g.add_edge(item[0], item[1])
    
    for item in n_col_reader:
        g.add_edge(item[0], item[1])
    print('done')
    pickle.dump(g, open('graph', 'wb'))
    return g

if not os.path.isfile('data/artist_states_names.csv'): make_a_s_csv()

graph = make_collab_graph()
state_bfs_trees = bfs_each_state(graph)

print_bfs_tree_stats(state_bfs_trees)
