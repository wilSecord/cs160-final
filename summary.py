import pandas as pd

artists = pd.read_csv("./query.csv")[['stateLabel', 'artist']]

states = artists.groupby(artists['stateLabel']).count()
#print(states)
#print(states.info())
states.sort_values('artist', inplace=True, ascending=False)
print(states.head(15))
