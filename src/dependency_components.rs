#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DependencyComponents {
    components: Vec<String>,
    is_certainly_internal: bool,
}

impl DependencyComponents {
    pub fn new(components: Vec<String>, is_certainly_internal: bool) -> Self {
        Self {
            components,
            is_certainly_internal,
        }
    }

    pub fn components<'a>(&'a self) -> &'a Vec<String> {
        &self.components
    }

    pub fn is_certainly_internal(&self) -> bool {
        self.is_certainly_internal
    }
}
