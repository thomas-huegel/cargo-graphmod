use crate::{trie::Trie, components::ModuleComponents};

pub type DependenciesGraph = Trie<String, Vec<ModuleComponents>>;
