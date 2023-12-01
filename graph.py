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


def make_collab_graph():
    g = nx.Graph()
    
    n_rels = open('data/artist_states_names.csv')
    n_collab = open('data/collaborations_names.csv')
    n_rel_reader = csv.reader(n_rels)
    n_col_reader = csv.reader(n_collab)

    for item in n_rel_reader:
        g.add_node(item[0], state=item[1])
    
    for item in n_col_reader:
        g.add_edge(item[0], item[1], weight=1)
    print('done')
    pickle.dump(g, open('graph', 'wb'))
make_collab_graph()
