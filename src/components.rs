use std::{collections::HashMap, fmt::Display};

pub const CRATE: &str = "crate";
pub const SEPARATOR: &str = "::";

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModuleComponents(pub Vec<String>);

impl From<Vec<String>> for ModuleComponents {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

impl Display for ModuleComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}{}\"", SEPARATOR, self.0.join(SEPARATOR))
    }
}

pub struct DependenciesGraph {
    pub map: HashMap<ModuleComponents, Vec<ModuleComponents>>,
}

impl DependenciesGraph {
    pub fn new() -> Self {
        DependenciesGraph { 
            map: HashMap::new(),
        }
    }
}