use std::collections::HashMap;

pub const CRATE: &str = "crate";

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModuleComponents(pub Vec<String>);

impl From<Vec<String>> for ModuleComponents {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

pub struct CodeBase(pub HashMap<ModuleComponents, String>);

impl CodeBase {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl From<HashMap<ModuleComponents, String>> for CodeBase {
    fn from(value: HashMap<ModuleComponents, String>) -> Self {
        Self(value)
    }
}