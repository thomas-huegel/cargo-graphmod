/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */
use std::path::Path;

use crate::{
    dependencies_graph::DependenciesGraph,
    dependencies_processor::rust_processor::target_computer::RustDependencyProcessor,
    files_reader,
    formatter::{dot_formatter::DotFormatter, Formatter},
    parser::rust_parser::RustParser,
};

pub fn run_app(directory: &str, pkg_name: &str) -> String {
    let path = Path::new(directory);
    let skip_length = path.iter().count();
    let mut trie = DependenciesGraph::new();
    files_reader::build_dependencies_trie::<RustParser>(path, &mut trie, skip_length).expect(
        "Unable to read ./src; please consider changing to the root directory of your package.",
    );
    DotFormatter::show::<RustDependencyProcessor>(&trie, pkg_name)
}
