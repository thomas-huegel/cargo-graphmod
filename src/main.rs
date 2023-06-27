use std::{env, path::Path, fs::read_to_string, io::Result, collections::{HashMap, HashSet}};

struct DependenciesGraph {
    set: HashSet<String>,
    map: HashMap<String, Vec<String>>,
}

impl DependenciesGraph {
    fn new() -> Self {
        DependenciesGraph { 
            set: HashSet::new(),
            map: HashMap::new(),
        }
    }
}

fn generate_graph(path: &Path, graph: &mut DependenciesGraph) -> Result<()> {
    if path.is_file() {
        if let Some(Some("rs")) = path.extension().map(|e| e.to_str()) {
            //println!("{:?}", path);
            let contents = read_to_string(path)?;
            let filename: String = path.to_string_lossy().into();
            graph.set.insert(filename.clone());
            graph.map.insert(filename.clone(), vec![]);
            for line in contents.lines() {
                let words = line.split_whitespace().collect::<Vec<&str>>();
                if let Some(&"use") = words.get(0) {
                    if let Some (&dependency) = words.get(1) {
                        graph.set.insert(dependency.into());
                        if let Some(dependencies) = graph.map.get_mut(&filename) {
                            dependencies.push(dependency.into());
                        }
                    }
                }
            }
        }
    } else if path.is_dir() {
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                generate_graph(&entry.path(), graph)?;
            }
        }
    }
    Ok(())
}

fn format_graph (graph: DependenciesGraph) -> String {
    let index: HashMap<String, i32> = graph.set.into_iter().zip(0..).collect();
    let vertices: String = index.iter()
        .map(|(name, idx)| String::from("u") + &idx.to_string() + "[label=\"" + name + "\"]")
        .collect::<Vec<String>>()
        .join("\n");
    let arcs: String = graph.map.into_iter()
        .map(|(k, values)| values.iter()
            .map(|v| String::from("u") + &index.get(&k).unwrap().to_string() + " -> " + "u" + &index.get(v).unwrap().to_string())
            .collect::<Vec<String>>()
            .join("\n"))
        .collect::<Vec<String>>()
        .join("\n");    
    String::from("digraph G {\n") + &vertices + "\n" + &arcs + "\n}\n"
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(directory) => {
            let mut graph = DependenciesGraph::new();
            if let Err(_) = generate_graph (directory.as_ref(), &mut graph) {
                println!("Error when generating the graph.");
            }
            let output = format_graph (graph);
            println!("{}", output);
        }
        None => println!("Which directory?"),
    }
}
