#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModuleComponents(pub Vec<String>);

impl From<Vec<String>> for ModuleComponents {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}
