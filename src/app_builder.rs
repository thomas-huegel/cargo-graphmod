use std::path::Path;

use crate::{dependencies_graph::DependenciesGraph, files_reader, dot_formatter};

pub fn run_app(directory: &str, crate_name: &str) -> String {
    let path = Path::new(directory);
    let skip_length = path.iter().count();
    let mut trie = DependenciesGraph::new();
    files_reader::read_files(path, &mut trie, skip_length, &crate_name).expect("Unable to read code base!");
    dot_formatter::show(&trie)
}