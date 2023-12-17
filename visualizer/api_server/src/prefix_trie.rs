use std::{borrow::Borrow, collections::VecDeque, iter::empty};

pub struct PrefixTrie<C, V> {
    branch_chars: Vec<C>,
    branches: Vec<PrefixTrie<C, V>>,
    values: Vec<V>,
}

impl<C, V> Default for PrefixTrie<C, V> {
    fn default() -> Self {
        Self {
            branch_chars: vec![],
            branches: vec![],
            values: vec![],
        }
    }
}

impl<C: Borrow<char> + PartialEq, V> PrefixTrie<C, V> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set<K: IntoIterator<Item = C>>(&mut self, key: K, value: V) {
        let mut trie = self;

        for ch in key {
            let index = trie.branch_chars.iter().position(|x| *x == ch);
            trie = if let Some(index) = index {
                trie.branches.get_mut(index).unwrap()
            } else {
                trie.branch_chars.push(ch);
                trie.branches.push(PrefixTrie::default());
                trie.branches.get_mut(trie.branch_chars.len() - 1).unwrap()
            }
        }

        trie.values = vec![value];
    }

    pub fn insert<K: IntoIterator<Item = C>>(&mut self, key: K, value: V) {
        let mut trie = self;

        for ch in key {
            let index = trie.branch_chars.iter().position(|x| *x == ch);
            trie = if let Some(index) = index {
                trie.branches.get_mut(index).unwrap()
            } else {
                trie.branch_chars.push(ch);
                trie.branches.push(PrefixTrie::default());
                trie.branches.get_mut(trie.branch_chars.len() - 1).unwrap()
            }
        }

        trie.values.push(value);
    }

    pub fn prefix_values<'a, K: IntoIterator<Item = C>>(
        &'a self,
        key: K,
    ) -> impl Iterator<Item = &'a V> {
        let mut trie = self;

        for ch in key {
            let index = trie.branch_chars.iter().position(|x| *x == ch);
            trie = if let Some(index) = index {
                trie.branches.get(index).unwrap()
            } else {
                return TrieValues::empty();
            }
        }

        trie.values()
    }

    pub fn values<'a>(&'a self) -> TrieValues<'a, C, V> {
        return TrieValues::<'a, C, V> {
            current_branches: [].iter(),
            current_values: [].iter(),
            tries: vec![self].into(),
        };
    }
}

impl<K: IntoIterator<Item = C>, C: Borrow<char> + PartialEq, V> FromIterator<(K, V)>
    for PrefixTrie<C, V>
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut trie = PrefixTrie::new();

        for (k, v) in iter {
            trie.insert(k, v);
        }

        trie
    }
}

pub struct TrieValues<'a, C, V> {
    tries: VecDeque<&'a PrefixTrie<C, V>>,
    current_branches: std::slice::Iter<'a, PrefixTrie<C, V>>,
    current_values: std::slice::Iter<'a, V>,
}
impl<'a, C, V> TrieValues<'a, C, V> {
    fn empty() -> Self {
        Self {
            tries: vec![].into(),
            current_branches: [].iter(),
            current_values: [].iter(),
        }
    }
}

impl<'a, C, V> Iterator for TrieValues<'a, C, V> {
    type Item = &'a V;

    fn next<'b>(&'b mut self) -> Option<Self::Item> {
        loop {
            match self.current_values.next() {
                Some(v) => return Some(v),
                None => {}
            }

            match self.current_branches.next() {
                Some(trie) => {
                    self.tries.push_back(trie);
                }
                None => {}
            }
            match self.tries.pop_front() {
                Some(PrefixTrie {
                    branches, values, ..
                }) => {
                    self.current_branches = branches.iter();
                    self.current_values = values.iter();
                }
                None => return None,
            }
        }
    }
}
