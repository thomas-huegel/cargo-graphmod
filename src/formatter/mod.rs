use crate::{dependencies_graph::DependenciesGraph, dependencies_processor::DependencyProcessor};

mod colors;
pub mod dot_formatter;

pub trait Formatter {
    fn show<Processor: DependencyProcessor>(trie: &DependenciesGraph, pkg_name: &str) -> String;
}
