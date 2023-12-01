use std::collections::VecDeque;

pub struct PrefixTrie<C, V> {
    branch_chars: Vec<C>,
    branches: Vec<PrefixTrie<C, V>>,
    value: Option<V>,
}

impl<C, V> Default for PrefixTrie<C, V> {
    fn default() -> Self {
        Self {
            branch_chars: vec![],
            branches: vec![],
            value: None,
        }
    }
}

impl<C: AsRef<char> + PartialEq, V> PrefixTrie<C, V> {
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

        trie.value = Some(value);
    }

    pub fn get_all_prefix<'a, K: IntoIterator<Item = C>>(&'a self, key: K) -> impl Iterator<Item = &'a V> {
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
            current_branches: &mut vec![].into_iter(),
            tries: vec![self].into(),
        };
    }
}

struct TrieValues<'a, C, V> {
    tries: VecDeque<&'a PrefixTrie<C, V>>,
    current_branches: &'a mut std::vec::IntoIter<PrefixTrie<C, V>>,
}
impl<'a, C, V> TrieValues<'a, C, V> {
    fn empty() -> Self {
        Self {
            tries: vec![].into(),
            current_branches: &mut vec![].into_iter()
        }
    }
}

impl<'a, C, V> Iterator for TrieValues<'a, C, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.current_branches.next() {
                Some(ref trie) => {
                    self.tries.push_back(trie);
                    if let Some(ref value) = trie.value {
                        return Some(value);
                    } else {
                        continue;
                    }
                }
                None => {
                    if let Some(next_trie) = self.tries.pop_front() {
                        self.current_branches = &mut next_trie.branches.into_iter();
                        continue;
                    } else {
                        return None;
                    }
                }
            }
        }
    }
}
