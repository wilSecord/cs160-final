import csv
import networkx as nx
import matplotlib.pyplot as plt
import pickle

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
                    frontier.push((neighbor_cost, neighbor))

                paths[neighbor] = (neighbor_cost, node)

    return paths 

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


# make_collab_graph()
