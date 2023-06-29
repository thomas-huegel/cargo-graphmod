use std::{collections::{HashMap}, fmt::Display};

use crate::components::{SEPARATOR, DependenciesGraph};

#[derive(Clone, PartialEq, Eq)]
pub struct ModulesTrie {
    path: String,
    basename: String,
    cluster_id: usize,
    children: HashMap<String, ModulesTrie>
}

impl Display for ModulesTrie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.children.is_empty() {
            write!(f, "\"{}\"[label=\"{}\"]\n", self.path, self.basename)?;
        } else {
            write!(f, "subgraph cluster_{} {{\n", self.cluster_id)?;
            write!(f, "label=\"{}\"\n", self.basename)?;
            write!(f, "\"{}\"[label=\"{}\"]\n", self.path, self.basename.to_string() + "::mod")?;
            for (_name, trie) in self.children.iter() {
                trie.fmt(f)?;
            }
            write!(f, "}}\n")?;
        }
        Ok(())
    }
}

fn insert (components: &[String], path: String, cluster_id: usize, trie: &mut ModulesTrie) -> usize {
    let n = components.len();
    match components.get(0) {
        None => cluster_id,
        Some(elt) => match trie.children.get_mut(elt) {
            None => {
                trie.children.insert(elt.to_string(), ModulesTrie { 
                    path: path.clone() + SEPARATOR + elt, 
                    basename: elt.to_string(), 
                    cluster_id: cluster_id, 
                    children: HashMap::new(),
                });
                let new_trie = trie.children.get_mut(elt).unwrap();
                insert(&components[1..n], path + SEPARATOR + elt, cluster_id + 1, new_trie)
            },
            Some(trie) => {
                insert(&components[1..n], path + SEPARATOR + elt, cluster_id + 1, trie)
            }
        }
    }
}

pub fn convert (dependencies_graph: &DependenciesGraph) -> ModulesTrie {
    let mut cluster_id = 0;
    let mut trie = ModulesTrie { 
        path: String::new(), 
        basename: String::new(),
        cluster_id: cluster_id,
        children: HashMap::new()
    };
    for elt in dependencies_graph.map.keys() {
        cluster_id = insert(&elt.0, String::new(), cluster_id + 1, &mut trie);
    }
    trie
}
