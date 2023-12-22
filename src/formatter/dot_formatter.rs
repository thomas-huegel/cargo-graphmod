/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */
use std::collections::BTreeSet as Set;

use crate::{
    dependencies_graph::DependenciesGraph,
    dependencies::{DependencyPath, FilePath},
    dependencies_processor::DependencyProcessor,
    formatter::{colors, Formatter},
};

const OUTPUT_SEPARATOR: &str = "::";
const CLUSTER_SEPARATOR: &str = "___";

fn cluster_id(path: &str) -> String {
    path.split(OUTPUT_SEPARATOR)
        .collect::<Vec<_>>()
        .join(CLUSTER_SEPARATOR)
}

fn show_vertices(trie: &DependenciesGraph, dirname: &str, basename: &str, level: usize) -> String {
    let path = if basename.is_empty() {
        String::new()
    } else {
        String::from(dirname) + OUTPUT_SEPARATOR + basename
    };
    let indentation = "  ".repeat(level);
    if trie.children.is_empty() {
        format!(
            "{}\"{}\"[label=\"{}\",style=\"filled\",fillcolor=\"{}\"]\n",
            indentation,
            path,
            basename,
            colors::make_random_color(dirname)
        )
    } else {
        format!("{}subgraph cluster_{} {{\n", indentation, cluster_id(&path))
            + &format!("{}label=\"{}\"\n", indentation, basename)
            + &format!("{}color=\"{}\"\n", indentation, colors::make_gray(level))
            + &format!("{}style=\"filled\"\n", indentation)
            + &trie
                .children
                .iter()
                .map(|(bname, trie)| show_vertices(trie, &path, bname, level + 1))
                .collect::<Vec<_>>()
                .join("")
            + &format!("{}}}\n", indentation)
    }
}

fn make_vertex(path: &FilePath) -> String {
    OUTPUT_SEPARATOR.to_owned() + &path.0.join(OUTPUT_SEPARATOR)
}

fn make_arrow<Processor: DependencyProcessor>(
    trie: &DependenciesGraph,
    current_path: &FilePath,
    dependency: &DependencyPath,
    pkg_name: &str,
) -> Option<String> {
    let target = Processor::compute_target(trie, current_path, dependency, pkg_name);
    if target.0.is_empty() {
        None
    } else {
        Some(
            String::from("\"")
                + &make_vertex(current_path)
                + "\" -> \""
                + &make_vertex(&target)
                + "\"",
        )
    }
}

fn show_dependencies_from_vertex<Processor: DependencyProcessor>(
    current_trie: &DependenciesGraph,
    whole_trie: &DependenciesGraph,
    current_path: &FilePath,
    pkg_name: &str,
) -> Option<String> {
    current_trie.value.as_ref().map(|dependencies| {
        dependencies
            .iter()
            .filter_map(|dependency| {
                make_arrow::<Processor>(whole_trie, current_path, dependency, pkg_name)
            })
            .collect::<Set<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .join("\n")
    })
}

fn show_arcs<Processor: DependencyProcessor>(
    current_trie: &DependenciesGraph,
    whole_trie: &DependenciesGraph,
    FilePath(path): &FilePath,
    pkg_name: &str,
) -> String {
    show_dependencies_from_vertex::<Processor>(
        current_trie,
        whole_trie,
        &FilePath(path.clone()),
        pkg_name,
    )
    .unwrap_or_default()
        + &current_trie
            .children
            .iter()
            .map(|(name, child)| {
                let mut new_path = path.clone();
                new_path.push(name.clone());
                show_arcs::<Processor>(child, whole_trie, &FilePath(new_path), pkg_name)
            })
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
}

pub struct DotFormatter {}

impl Formatter for DotFormatter {
    fn show<Processor: DependencyProcessor>(trie: &DependenciesGraph, pkg_name: &str) -> String {
        String::from("digraph dependencies {\n")
            + &show_vertices(trie, "", "", 0)
            + &show_arcs::<Processor>(trie, trie, &FilePath(vec![]), pkg_name)
            + "\n}\n"
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap as Map;

    use crate::{
        dependencies_graph::DependenciesGraph,
        dependencies::DependencyPath,
        dependencies_processor::rust_processor::target_computer::RustDependencyProcessor,
        formatter::{dot_formatter::DotFormatter, Formatter},
    };

    fn make_trie() -> DependenciesGraph {
        DependenciesGraph {
            value: None,
            children: Map::from([
                (
                    String::from("lib"),
                    DependenciesGraph {
                        value: None,
                        children: Map::new(),
                    },
                ),
                (
                    String::from("foo"),
                    DependenciesGraph {
                        value: None,
                        children: Map::from([
                            (
                                String::from("bar"),
                                DependenciesGraph {
                                    value: Some(vec![
                                        DependencyPath(vec![
                                            String::from("crate"),
                                            String::from("abc"),
                                        ]),
                                        DependencyPath(vec![String::from("std")]),
                                    ]),
                                    children: Map::new(),
                                },
                            ),
                            (
                                String::from("mod"),
                                DependenciesGraph {
                                    value: Some(vec![DependencyPath(vec![
                                        String::from("bar"),
                                        String::from("baz"),
                                    ])]),
                                    children: Map::new(),
                                },
                            ),
                        ]),
                    },
                ),
                (
                    String::from("abc"),
                    DependenciesGraph {
                        value: Some(vec![
                            DependencyPath(vec![
                                String::from("crate"),
                                String::from("foo"),
                                String::from("Panel"),
                            ]),
                            DependencyPath(vec![String::from("crate"), String::from("Widget")]),
                        ]),
                        children: Map::new(),
                    },
                ),
                (
                    String::from("def"),
                    DependenciesGraph {
                        value: Some(vec![DependencyPath(vec![
                            String::from("crate"),
                            String::from("foo"),
                            String::from("bar"),
                            String::from("Widget"),
                        ])]),
                        children: Map::new(),
                    },
                ),
            ]),
        }
    }

    #[test]
    fn it_outputs_to_dot() {
        let trie = make_trie();
        let result = DotFormatter::show::<RustDependencyProcessor>(&trie, "my_crate");
        let expected = String::from(
            r##"digraph dependencies {
subgraph cluster_ {
label=""
color="#ffffff"
style="filled"
  "::abc"[label="abc",style="filled",fillcolor="#ffffff"]
  "::def"[label="def",style="filled",fillcolor="#ffffff"]
  subgraph cluster____foo {
  label="foo"
  color="#eeeeee"
  style="filled"
    "::foo::bar"[label="bar",style="filled",fillcolor="#86c2dc"]
    "::foo::mod"[label="mod",style="filled",fillcolor="#86c2dc"]
  }
  "::lib"[label="lib",style="filled",fillcolor="#ffffff"]
}
"::abc" -> "::foo::mod"
"::abc" -> "::lib"
"::def" -> "::foo::bar"
"::foo::bar" -> "::abc"
"::foo::bar" -> "::std"
"::foo::mod" -> "::foo::bar"
}
"##,
        );
        assert_eq!(result, expected);
    }
}
