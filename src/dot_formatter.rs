use std::collections::BTreeSet as Set;

use crate::{components::ModuleComponents, colors, dependencies_graph::DependenciesGraph};

const LIB: &str = "lib";
const MOD: &str = "mod";
const OUTPUT_SEPARATOR: &str = "::";
const CLUSTER_SEPARATOR: &str = "___";

fn cluster_id(path: &str) -> String {
    path.split(OUTPUT_SEPARATOR).collect::<Vec<_>>().join(CLUSTER_SEPARATOR)
}

fn show_vertices (trie: &DependenciesGraph, dirname: &str, basename: &str, level: usize) -> String {
    let path = if basename.is_empty() {
        String::new()
    } else {
        String::from(dirname) + OUTPUT_SEPARATOR + basename
    };
    let indentation = "  ".repeat(level);
    if trie.children.is_empty() {
        format!("{}\"{}\"[label=\"{}\",style=\"filled\",fillcolor=\"{}\"]\n", indentation, path, basename, colors::make_random_color(dirname))
    } else {
        format!("{}subgraph cluster_{} {{\n", indentation, cluster_id(&path))
        + &format!("{}label=\"{}\"\n", indentation, basename)
        + &format!("{}color=\"{}\"\n", indentation, colors::make_gray(level))
        + &format!("{}style=\"filled\"\n", indentation)
        + &trie.children.iter()
            .map(|(bname, trie)|
                show_vertices(trie, &path, bname, level + 1))
            .collect::<Vec<_>>()
            .join("")
        + &format!("{}}}\n", indentation)
    }
}

fn make_arrow(trie: &DependenciesGraph, path: &str, v: &ModuleComponents) -> String {
    let (longest_prefix, value) = trie.get_longest_prefix(&v.0);
    let target = if longest_prefix.is_empty() {
        OUTPUT_SEPARATOR.to_owned() + LIB
    } else {
        let appendix = match value {
            None => OUTPUT_SEPARATOR.to_owned() + MOD,
            Some(_) => "".to_string()
        };
        OUTPUT_SEPARATOR.to_owned() + &longest_prefix.join(OUTPUT_SEPARATOR) + &appendix
    };
    String::from("\"") + path + "\" -> \"" + &target + "\""

}

fn show_dependencies_from_vertex(current_trie: &DependenciesGraph, whole_trie: &DependenciesGraph, path: &str) -> Option<String> {
    current_trie.value.as_ref().map (|deps| deps.iter()
        .map(|v| make_arrow(whole_trie, path, v))
        .collect::<Set<_>>()
        .into_iter()
        .collect::<Vec<_>>()
        .join("\n")
    )
}

fn show_arcs (current_trie: &DependenciesGraph, whole_trie: &DependenciesGraph, path: &str) -> String {
    show_dependencies_from_vertex(current_trie, whole_trie, path).unwrap_or(String::new())
    +
    &current_trie.children.iter()
    .map(|(name, child)| show_arcs(child, whole_trie, &(String::from(path) + OUTPUT_SEPARATOR + name)))
    .filter(|s| s != "")
    .collect::<Vec<_>>()
    .join("\n")
}

pub fn show(trie: &DependenciesGraph) -> String {
    String::from("digraph dependencies {\n")
        + &show_vertices(trie, "", "", 0)
        + &show_arcs(trie, trie, "")
        + "\n}\n"
}


#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap as Map};

    use crate::{components::ModuleComponents, dot_formatter::show, dependencies_graph::DependenciesGraph};
    
    #[test]
    fn it_outputs_to_dot() {
        let trie = DependenciesGraph { 
            value: None, 
            children: Map::from([
                (String::from("lib"), DependenciesGraph {
                    value: None,
                    children: Map::new(),
                }),
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
                }),
                (String::from("ghi"), DependenciesGraph {
                    value: Some(vec![ModuleComponents(vec![String::from("Widget")])]),
                    children: Map::new()
                }),
            ])
        };
        let result = show(&trie);
        let expected = String::from(
r###"digraph dependencies {
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
  "::ghi"[label="ghi",style="filled",fillcolor="#ffffff"]
  "::lib"[label="lib",style="filled",fillcolor="#ffffff"]
}
"::abc" -> "::foo::mod"
"::def" -> "::foo::bar"
"::foo::bar" -> "::abc"
"::foo::bar" -> "::def"
"::ghi" -> "::lib"
}
"###
        );
        assert_eq!(result, expected);
    }
}
