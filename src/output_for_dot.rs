use std::collections::BTreeSet as Set;

use crate::components::{DependenciesGraph, ModuleComponents};

const MODULE: &str = "mod";
const OUTPUT_SEPARATOR: &str = "::";
const CLUSTER_SEPARATOR: &str = "___";

fn cluster_id(path: &str) -> String {
    path.split(OUTPUT_SEPARATOR).collect::<Vec<_>>().join(CLUSTER_SEPARATOR)
}

fn show_vertices (trie: &DependenciesGraph, path: &str, basename: &str) -> String {
    if trie.children.is_empty() {
        format!("\"{}\"[label=\"{}\"]\n", path, basename)
    } else {
        let module_node = if path.is_empty() {
            String::new()
        } else {
            format!("\"{}\"[label=\"{}\"]\n", path, basename.to_string() + OUTPUT_SEPARATOR + MODULE)
        };
        format!("subgraph cluster_{} {{\n", cluster_id(path))
        + &format!("label=\"{}\"\n", basename)
        + &module_node
        + &trie.children.iter()
            .map(|(bname, trie)| show_vertices(trie, &(path.to_string() + OUTPUT_SEPARATOR + bname), bname))
            .collect::<Vec<_>>()
            .join("")
        + "}\n"
    }
}

fn make_arrow(trie: &DependenciesGraph, path: &str, v: &ModuleComponents) -> Option<String> {
    let longest_prefix = trie.get_longest_prefix(&v.0);
    if longest_prefix.is_empty() {
        None
    } else {
        Some (String::from("\"") + path + "\" -> \"" + OUTPUT_SEPARATOR + &longest_prefix.join(OUTPUT_SEPARATOR) + "\"")
    }
}

fn show_dependencies_from_vertex(current_trie: &DependenciesGraph, whole_trie: &DependenciesGraph, path: &str) -> Option<String> {
    current_trie.value.as_ref().map (|deps| deps.iter()
        .filter_map(|v| make_arrow(whole_trie, path, v))
        .collect::<Set<_>>()
        .into_iter()
        .collect::<Vec<_>>()
        .join("\n")
    )
}

fn show_arcs (current_trie: &DependenciesGraph, whole_trie: &DependenciesGraph, path: &str) -> String {
    if current_trie.children.is_empty() {
        return show_dependencies_from_vertex(current_trie, whole_trie, path).unwrap_or(String::new())
    }
    current_trie.children.iter()
    .map(|(name, child)| show_arcs(child, whole_trie, &(String::from(path) + OUTPUT_SEPARATOR + name)))
    .filter(|s| s != "")
    .collect::<Vec<_>>()
    .join("\n")
}

pub fn show(trie: &DependenciesGraph) -> String {
    String::from("digraph dependencies {\n")
        + &show_vertices(trie, "", "")
        + &show_arcs(trie, trie, "")
        + "\n}\n"
}


#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap as Map};

    use crate::{components::DependenciesGraph, components::ModuleComponents, output_for_dot::show};
    
    #[test]
    fn it_outputs_to_dot() {
        let trie = DependenciesGraph { 
            value: None, 
            children: Map::from([
                (String::from("foo"), DependenciesGraph { 
                    value: None,
                    children: Map::from([
                        (String::from("bar"), DependenciesGraph {
                            value: Some(vec![ModuleComponents(vec![String::from("abc")]), ModuleComponents(vec![String::from("def")])]),
                            children: Map::new()
                        }),
                        (String::from("mod"), DependenciesGraph { 
                            value: Some(vec![]),
                            children: Map::new()
                        })
                    ])
                }),
                (String::from("abc"), DependenciesGraph {
                    value: Some(vec![ModuleComponents(vec![String::from("foo"), String::from("Panel")])]),
                    children: Map::new()
                }),
                (String::from("def"), DependenciesGraph {
                    value: Some(vec![ModuleComponents(vec![String::from("foo"), String::from("bar"), String::from("Widget")])]),
                    children: Map::new()
                })
            ])
        };
        let result = show(&trie);
        let expected = String::from(
r#"digraph dependencies {
subgraph cluster_ {
label=""
"::abc"[label="abc"]
"::def"[label="def"]
subgraph cluster____foo {
label="foo"
"::foo"[label="foo::mod"]
"::foo::bar"[label="bar"]
"::foo::mod"[label="mod"]
}
}
"::abc" -> "::foo"
"::def" -> "::foo::bar"
"::foo::bar" -> "::abc"
"::foo::bar" -> "::def"
}
"#
        );
        assert_eq!(result, expected);
    }
}
