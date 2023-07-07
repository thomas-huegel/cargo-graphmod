use std::path::Path;

use crate::{dependencies_graph::DependenciesGraph, files_reader, dot_formatter};

pub fn run_app(directory: &str, pkg_name: &str) -> String {
    let path = Path::new(directory);
    let skip_length = path.iter().count();
    let mut trie = DependenciesGraph::new();
    files_reader::read_files(path, &mut trie, skip_length, pkg_name).expect(
        "Unable to read ./src; please consider changing to the root directory of your package."
    );
    dot_formatter::show(&trie)
}