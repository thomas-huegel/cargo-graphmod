use crate::trie::Trie;

pub const CRATE: &str = "crate";

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModuleComponents(pub Vec<String>);

impl From<Vec<String>> for ModuleComponents {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

pub type DependenciesGraph = Trie<String, Vec<ModuleComponents>>;
