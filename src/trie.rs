use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub struct Trie<'a, K: Eq + Hash, V> {
    pub value: Option<V>,
    pub children: HashMap<&'a K, Trie<'a, K, V>>
}

impl<'a, K: Eq + Hash, V: Clone> Trie<'a, K, V> {
    pub fn new() -> Self {
        Self {
            value: None,
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: &'a[K], v: V) -> Option<V> {
        let n = k.len();
        match k.get(0) {
            None => {
                let old_value = self.value.clone();
                self.value = Some(v);
                old_value
            }
            Some(elt) => match self.children.get_mut(elt) {
                None => {
                    self.children.insert(elt, Trie::new());
                    let new_trie = self.children.get_mut(elt).unwrap();
                    new_trie.insert(&k[1..n], v)
                },
                Some(trie) => {
                    trie.insert(&k[1..n], v)
                }
            }
        }
    }

    pub fn get_longest_prefix (&self, k: &'a[K]) -> Option<&'a[K]> {
        let n = k.len();
        let mut bound = 0;
        let mut trie = self;
        while let Some(child) = trie.children.get(&k[bound]) {
            trie = child;
            bound = bound + 1;
            if bound == n {
                break;
            }
        }
        if bound == 0 {
            None
        } else {
            Some(&k[0..bound])
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::trie::Trie;

    #[test]
    fn it_builds_a_one_branch_trie() {
        let mut trie = Trie::new();
        let array = [1,2];
        trie.insert(&array, 10);
        assert_eq!(trie, Trie {
            value: None,
            children: HashMap::from([(&1, Trie {
                value: None,
                children: HashMap::from([(&2, Trie {
                    value: Some(10),
                    children: HashMap::new()
                })])
            })])
        });
    }

    #[test]
    fn it_builds_a_two_branch_trie() {
        let mut trie = Trie::new();
        let array2 = [1,2];
        let array3 = [1,3];
        trie.insert(&array2, 20);
        trie.insert(&array3, 30);
        assert_eq!(trie, Trie {
            value: None,
            children: HashMap::from([(&1, Trie {
                value: None,
                children: HashMap::from([
                    (&2, Trie {
                        value: Some(20),
                        children: HashMap::new()
                    }),
                    (&3, Trie {
                        value: Some(30),
                        children: HashMap::new()
                    }),                    ])
            })])
        });
    }

    #[test]
    fn it_computes_the_longest_prefix() {
        let mut trie = Trie::new();
        let array2 = [1,2];
        let array3 = [1,3];
        trie.insert(&array2, 20);
        trie.insert(&array3, 30);
        assert_eq!(trie.get_longest_prefix(&[1,3,4]), Some(&array3[0..2]));
    }
}