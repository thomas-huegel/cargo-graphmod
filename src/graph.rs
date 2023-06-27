use std::{collections::{HashSet, HashMap}, path::Path, fs::read_to_string, io::Result};

type ModuleComponents = Vec<String>;
pub struct DependenciesGraph {
    set: HashSet<ModuleComponents>,
    map: HashMap<ModuleComponents, Vec<ModuleComponents>>,
}

impl DependenciesGraph {
    pub fn new() -> Self {
        DependenciesGraph { 
            set: HashSet::new(),
            map: HashMap::new(),
        }
    }
}

fn handle_dependency_line(line: &str, crate_name: &str) -> Option<ModuleComponents> {
    let words = line.split_whitespace().collect::<Vec<&str>>();
    if let Some(&"use") = words.get(0) {
        if let Some (&dependency) = words.get(1) {
            let mut dep = dependency.to_string();
            dep.pop();
            let dependency_components: ModuleComponents = dep.split("::").map(|s| s.into()).collect();
            if let Some(fst) = dependency_components.get(0) {
                if fst == crate_name || fst == "crate" {
                    return Some(dependency_components.iter().skip(1).map(|s| s.into()).collect::<Vec<String>>());
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
            let components = path.with_extension("").iter().skip(skip_length).map(|s| s.to_string_lossy().into()).collect::<Vec<String>>();
            graph.set.insert(components.clone());
            graph.map.insert(components.clone(), contents.lines().filter_map(|line| handle_dependency_line(line, crate_name)).collect());
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

pub fn format_graph (graph: DependenciesGraph) -> String {
    let index: HashMap<ModuleComponents, i32> = graph.set.into_iter().zip(0..).collect();
    let vertices: String = index.iter()
        .map(|(components, idx)| String::from("u") + &idx.to_string() + "[label=\"" + &components.join("::") + "\"]")
        .collect::<ModuleComponents>()
        .join("\n");
    let arcs: String = graph.map.into_iter()
        .map(|(k, values)| values.iter()
            .filter_map(|v| index.get(v).map(|idx| String::from("u") + &index.get(&k).unwrap().to_string() + " -> " + "u" + &idx.to_string()))
            .collect::<ModuleComponents>()
            .join("\n"))
        .filter(|s| s != "")
        .collect::<ModuleComponents>()
        .join("\n");    
    String::from("digraph G {\n\n") + &vertices + "\n" + &arcs + "\n}\n"
}