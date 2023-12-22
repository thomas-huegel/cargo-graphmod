use crate::{
    dependencies::{DependencyPath, FilePath},
    dependencies_graph::DependenciesGraph,
};

pub mod rust_processor;

pub trait DependencyProcessor {
    fn compute_target(
        trie: &DependenciesGraph,
        file_path: &FilePath,
        dependency: &DependencyPath,
        pkg_name: &str,
    ) -> FilePath;
}
