use std::{path::Path, fs::read_to_string, io::Result, collections::HashSet};

use crate::{components::{ModuleComponents, DependenciesGraph, SEPARATOR}, modules_trie};

fn handle_dependency_line(line: &str, current_path: &str, crate_name: &str) -> Option<ModuleComponents> {
    let mut words = line.split_whitespace();
    if let Some(first) = words.next() {
        if let Some(second) = words.next() {
            let mut dependency = if first == "pub" && second == "use" {
                crate_name.to_string() + SEPARATOR + current_path + SEPARATOR + words.next().unwrap()
            } else {
                second.to_string()
            };
            if first == "use" || (first == "pub" && second == "use") {
                dependency.pop();
                //println!("*** {} ***", dependency);
                let dependency_components: ModuleComponents = dependency.split("::").map(|s| s.into()).collect::<Vec<_>>().into();
                if let Some(fst) = dependency_components.0.get(0) {
                    if fst == crate_name || fst == "crate" {
                        return Some(dependency_components.0.iter().skip(1).map(|s| s.into()).collect::<Vec<String>>().into());
                    }
                }
            }
        }
    }
    None
}

pub fn generate_graph(path: &Path, graph: &mut DependenciesGraph, crate_name: &str, skip_length: usize) -> Result<()> {
    if path.is_file() {
        if let Some(Some("rs")) = path.extension().map(|e| e.to_str()) {
            let contents = read_to_string(path)?;
            let components: ModuleComponents = path.with_extension("").iter()
                .skip(skip_length)
                .map(|s| s.to_string_lossy().into())
                .filter(|s| s != "mod")
                .collect::<Vec<String>>().into();
            /*if components.0.last() == Some (&String::from("mod")) {
                    components.0.pop();
                }*/
            //println!("{:?}", components.0);
            graph.map.insert(
                components.clone(), 
                contents.lines().filter_map(|line| handle_dependency_line(line, &components.0.join("::"), crate_name))
            .collect());
        }
    } else if path.is_dir() {
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                generate_graph(&entry.path(), graph, crate_name, skip_length)?;
            }
        }
    }
    Ok(())
}

fn lookup (dependency: &ModuleComponents, graph: &DependenciesGraph) -> Option<ModuleComponents> {
    let mut components = dependency.clone();
    while graph.map.get(&components).is_none() && ! components.0.is_empty() {
        components.0.pop();
    }
    if components.0.is_empty() {
        None
    } else {
        Some (components)
    }
}

pub fn format_graph (graph: DependenciesGraph) -> String {
    //let index: ModulesIndex = graph.set.into_iter().zip((0..).map(Index)).collect::<HashMap<_,_>>().into();
    let trie = modules_trie::convert(&graph);
    /* let vertices: String = index.0.iter()
        .map(|(components, idx)| String::from("u") + &idx.0.to_string() + "[label=\"" + &components.0.join("::") + "\"]")
        .collect::<Vec<_>>()
        .join("\n"); */
    let arcs: String = graph.map.iter()
        .map(|(k, values)| values.iter()
            //.filter_map(|v| index.get(v).map(|idx| String::from("u") + &index.get(&k).unwrap().to_string() + " -> " + "u" + &idx.to_string()))
            .filter_map(|v| lookup(v, &graph).map(|trimmed| format!("{}", k) + " -> " + &format!("{}", trimmed)))
            //.map(|v| format!("{}", k) + " -> " + &format!("{}", v))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .join("\n"))
        .filter(|s| s != "")
        .collect::<Vec<_>>()
        .join("\n");    
    String::from("digraph G {\n\n") + &format!("{}", trie) + &arcs + "\n}\n"
}