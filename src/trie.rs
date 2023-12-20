/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */
use std::collections::{BTreeMap as Map, VecDeque};

#[derive(Debug, PartialEq, Eq)]
pub struct Trie<K: Eq + Ord, V> {
    pub value: Option<V>,
    pub children: Map<K, Trie<K, V>>,
}

impl<K: Eq + Ord + Clone, V: Clone> Trie<K, V> {
    pub fn new() -> Self {
        Self {
            value: None,
            children: Map::new(),
        }
    }

    pub fn insert(&mut self, mut k: VecDeque<K>, v: V) -> Option<V> {
        match k.pop_front() {
            None => {
                let old_value = self.value.clone();
                self.value = Some(v);
                old_value
            }
            Some(elt) => match self.children.get_mut(&elt) {
                None => {
                    self.children.insert(elt.clone(), Trie::new());
                    let new_trie = self.children.get_mut(&elt).unwrap();
                    new_trie.insert(k, v)
                }
                Some(trie) => trie.insert(k, v),
            },
        }
    }

    pub fn get_longest_prefix<'a>(&self, k: &'a [K]) -> (&'a [K], Option<V>) {
        let n = k.len();
        let mut bound = 0;
        let mut trie = self;
        while let Some(child) = trie.children.get(&k[bound]) {
            trie = child;
            bound += 1;
            if bound == n {
                break;
            }
        }
        (&k[0..bound], trie.value.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap as Map, VecDeque};

    use crate::trie::Trie;

    #[test]
    fn it_builds_a_one_branch_trie() {
        let mut trie = Trie::new();
        let k = VecDeque::from([1, 2]);
        trie.insert(k, 10);
        assert_eq!(
            trie,
            Trie {
                value: None,
                children: Map::from([(
                    1,
                    Trie {
                        value: None,
                        children: Map::from([(
                            2,
                            Trie {
                                value: Some(10),
                                children: Map::new()
                            }
                        )])
                    }
                )])
            }
        );
    }

    #[test]
    fn it_builds_a_two_branch_trie() {
        let mut trie = Trie::new();
        let k2 = VecDeque::from([1, 2]);
        let k3 = VecDeque::from([1, 3]);
        trie.insert(k2, 20);
        trie.insert(k3, 30);
        assert_eq!(
            trie,
            Trie {
                value: None,
                children: Map::from([(
                    1,
                    Trie {
                        value: None,
                        children: Map::from([
                            (
                                2,
                                Trie {
                                    value: Some(20),
                                    children: Map::new()
                                }
                            ),
                            (
                                3,
                                Trie {
                                    value: Some(30),
                                    children: Map::new()
                                }
                            ),
                        ])
                    }
                )])
            }
        );
    }

    #[test]
    fn it_computes_the_longest_prefix() {
        let mut trie = Trie::new();
        let k2 = VecDeque::from([1, 2]);
        let k3 = VecDeque::from([1, 3]);
        trie.insert(k2, 20);
        trie.insert(k3, 30);
        let a1 = [1, 3, 4];
        let a2 = [1, 4];
        let a3 = [4];
        assert_eq!(trie.get_longest_prefix(&a1), (&a1[0..2], Some(30)));
        assert_eq!(trie.get_longest_prefix(&a2), (&a2[0..1], None));
        assert_eq!(trie.get_longest_prefix(&a3), (&a3[0..0], None));
    }
}
