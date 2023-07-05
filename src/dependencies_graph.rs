use crate::{trie::Trie, dependency_components::DependencyComponents};

pub type DependenciesGraph = Trie<String, Vec<DependencyComponents>>;
