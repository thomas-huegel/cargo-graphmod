use crate::{
    dependencies::{DependencyPath, FilePath},
    dependencies_graph::DependenciesGraph,
    dependencies_processor::DependencyProcessor,
    trie::NodeKind,
};

use super::{dependency_expander::expand_dependency, DependencyKind, MOD};

const LIB: &str = "lib";

pub struct RustDependencyProcessor {}

impl DependencyProcessor for RustDependencyProcessor {
    fn compute_target(
        trie: &DependenciesGraph,
        FilePath(file_path): &FilePath,
        DependencyPath(dependency): &DependencyPath,
        pkg_name: &str,
    ) -> FilePath {
        let dependency = expand_dependency(dependency, pkg_name, file_path.clone());
        let (longest_prefix, node_kind) = trie.get_longest_prefix(&dependency.path.0);
        let longest_prefix_len = longest_prefix.len();
        let mut longest_prefix = Vec::from(longest_prefix);
        if node_kind == NodeKind::Internal {
            longest_prefix.push(MOD.into());
        }
        let target = match dependency.kind {
            DependencyKind::Relative => {
                if longest_prefix.is_empty() {
                    vec![LIB.into()]
                } else {
                    longest_prefix
                }
            }
            DependencyKind::Ambiguous(source_file_path_len) => {
                if longest_prefix_len <= source_file_path_len {
                    // external dependency
                    dependency.path.0[source_file_path_len..(source_file_path_len + 1)].into()
                } else {
                    // inner relative dependency
                    longest_prefix
                }
            }
        };
        FilePath(target)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap as Map;

    use crate::{
        dependencies_graph::DependenciesGraph,
        dependencies::{DependencyPath, FilePath},
        dependencies_processor::{
            rust_processor::target_computer::RustDependencyProcessor, DependencyProcessor,
        },
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
                                    value: None,
                                    children: Map::new(),
                                },
                            ),
                            (
                                String::from("mod"),
                                DependenciesGraph {
                                    value: None,
                                    children: Map::new(),
                                },
                            ),
                        ]),
                    },
                ),
                (
                    String::from("abc"),
                    DependenciesGraph {
                        value: None,
                        children: Map::new(),
                    },
                ),
                (
                    String::from("def"),
                    DependenciesGraph {
                        value: None,
                        children: Map::new(),
                    },
                ),
            ]),
        }
    }

    #[test]
    fn it_targets_lib() {
        let trie = make_trie();
        let file_path = FilePath(vec![String::from("foo"), String::from("bar")]);
        let dependency = DependencyPath(vec![String::from("crate"), String::from("Widget")]);
        let pkg_name = "my_crate";
        assert_eq!(
            RustDependencyProcessor::compute_target(&trie, &file_path, &dependency, pkg_name),
            FilePath(vec![String::from("lib")])
        );
    }

    #[test]
    fn it_targets_mod() {
        let trie = make_trie();
        let file_path = FilePath(vec![String::from("abc")]);
        let dependency = DependencyPath(vec![
            String::from("crate"),
            String::from("foo"),
            String::from("Widget"),
        ]);
        let pkg_name = "my_crate";
        assert_eq!(
            RustDependencyProcessor::compute_target(&trie, &file_path, &dependency, pkg_name),
            FilePath(vec![String::from("foo"), String::from("mod")])
        );
    }

    #[test]
    fn it_targets_some_child_directly() {
        let trie = make_trie();
        let file_path = FilePath(vec![String::from("foo"), String::from("mod")]);
        let dependency = DependencyPath(vec![String::from("bar"), String::from("baz")]);
        let pkg_name = "my_crate";
        assert_eq!(
            RustDependencyProcessor::compute_target(&trie, &file_path, &dependency, pkg_name),
            FilePath(vec![String::from("foo"), String::from("bar")])
        );
    }

    #[test]
    fn it_targets_some_child_via_self() {
        let trie = make_trie();
        let file_path = FilePath(vec![String::from("foo"), String::from("mod")]);
        let dependency = DependencyPath(vec![
            String::from("self"),
            String::from("bar"),
            String::from("baz"),
        ]);
        let pkg_name = "my_crate";
        assert_eq!(
            RustDependencyProcessor::compute_target(&trie, &file_path, &dependency, pkg_name),
            FilePath(vec![String::from("foo"), String::from("bar")])
        );
    }

    #[test]
    fn it_targets_some_sibling_via_pkg_name() {
        let trie = make_trie();
        let file_path = FilePath(vec![String::from("foo"), String::from("bar")]);
        let dependency = DependencyPath(vec![
            String::from("my_crate"),
            String::from("abc"),
            String::from("def"),
        ]);
        let pkg_name = "my_crate";
        assert_eq!(
            RustDependencyProcessor::compute_target(&trie, &file_path, &dependency, pkg_name),
            FilePath(vec![String::from("abc")])
        );
    }

    #[test]
    fn it_targets_some_sibling_via_crate() {
        let trie = make_trie();
        let file_path = FilePath(vec![String::from("foo"), String::from("bar")]);
        let dependency = DependencyPath(vec![
            String::from("crate"),
            String::from("abc"),
            String::from("def"),
        ]);
        let pkg_name = "my_crate";
        assert_eq!(
            RustDependencyProcessor::compute_target(&trie, &file_path, &dependency, pkg_name),
            FilePath(vec![String::from("abc")])
        );
    }

    #[test]
    fn it_targets_some_sibling_via_super() {
        let trie = make_trie();
        let file_path = FilePath(vec![String::from("foo"), String::from("bar")]);
        let dependency = DependencyPath(vec![
            String::from("super"),
            String::from("super"),
            String::from("abc"),
            String::from("def"),
        ]);
        let pkg_name = "my_crate";
        assert_eq!(
            RustDependencyProcessor::compute_target(&trie, &file_path, &dependency, pkg_name),
            FilePath(vec![String::from("abc")])
        );
    }

    #[test]
    fn it_targets_an_external_dependency() {
        let trie = make_trie();
        let file_path = FilePath(vec![String::from("abc")]);
        let dependency = DependencyPath(vec![String::from("std")]);
        let pkg_name = "my_crate";
        assert_eq!(
            RustDependencyProcessor::compute_target(&trie, &file_path, &dependency, pkg_name),
            FilePath(vec![String::from("std")])
        );
    }
}
