use std::collections::HashSet;

use crate::{dependencies_graph::DependenciesGraph};

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
        format!("subgraph cluster_{} {{\n", cluster_id(path))
        + &format!("label=\"{}\"\n", basename)
        + &format!("\"{}\"[label=\"{}\"]\n", path, basename.to_string() + OUTPUT_SEPARATOR + MODULE)
        + &trie.children.iter()
            .map(|(bname, trie)| show_vertices(trie, &(path.to_string() + OUTPUT_SEPARATOR + bname), bname))
            .collect::<Vec<_>>()
            .join("")
        + "}\n"
    }
}

fn show_dependencies_from_vertex(trie: &DependenciesGraph, path: &str) -> Option<String> {
    trie.value.as_ref().map (|deps: &Vec<crate::components::ModuleComponents>| deps.iter()
        .filter_map(|v| trie.get_longest_prefix(&v.0)
            .map(|trimmed| String::from(path) + " -> " + &trimmed.join(OUTPUT_SEPARATOR))
        )
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
        .join("\n")
    )
}

fn show_arcs (trie: &DependenciesGraph, path: &str) -> String {
    if trie.children.is_empty() {
        return show_dependencies_from_vertex(trie, path).unwrap_or(String::new())
    }
    trie.children.iter()
    .map(|(name, child)| show_arcs(child, &(String::from(path) + OUTPUT_SEPARATOR + name)))
    .filter(|s| s != "")
    .collect::<Vec<_>>()
    .join("\n")
}

pub fn show(trie: &DependenciesGraph) -> String {
    String::from("digraph dependencies {\n")
        + &show_vertices(trie, "", "")
        + &show_arcs(trie, "")
        + "}\n"
}


#[cfg(test)]
mod tests {
    use std::{collections::HashMap};

    use crate::{dependencies_graph::DependenciesGraph, components::ModuleComponents, output_for_dot::show};
    
    #[test]
    fn it_outputs_to_dot() {
        let sfoo = String::from("foo");
        let sbar = String::from("bar");
        let smod = String::from("mod");
        let sabc = String::from("abc");
        let sdef = String::from("def");
        let trie = DependenciesGraph { 
            value: None, 
            children: HashMap::from([
                (&sfoo, DependenciesGraph { 
                    value: None,
                    children: HashMap::from([
                        (&sbar, DependenciesGraph {
                            value: Some(vec![ModuleComponents(vec![String::from("abc")]), ModuleComponents(vec![String::from("def")])]),
                            children: HashMap::new()
                        }),
                        (&smod, DependenciesGraph { 
                            value: Some(vec![]),
                            children: HashMap::new()
                        })
                    ])
                }),
                (&sabc, DependenciesGraph {
                    value: Some(vec![ModuleComponents(vec![String::from("foo"), String::from("Panel")])]),
                    children: HashMap::new()
                }),
                (&sdef, DependenciesGraph {
                    value: Some(vec![ModuleComponents(vec![String::from("foo"), String::from("bar"), String::from("Widget")])]),
                    children: HashMap::new()
                })
            ])
        };
        let result = show(&trie);
        let expected = String::from(
r#"digraph dependencies {
subgraph cluster_ {
label=""
""[label="::mod"]
subgraph cluster____foo {
label="foo"
"::foo"[label="foo::mod"]
"::foo::bar"[label="bar"]
"::foo::mod"[label="mod"]
}
"::def"[label="def"]
"::abc"[label="abc"]
}
}
"#
        );
        assert_eq!(result, expected);
    }
}
