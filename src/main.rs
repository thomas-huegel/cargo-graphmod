use std::{env, path::Path};

use cargo_graphmod::graph;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(directory) => {
            let mut graph = graph::DependenciesGraph::new();
            match args.get(2) {
                Some(crate_name) => {
                    let path = Path::new(directory);
                    let skip_length = path.iter().count() + 1;
                    if graph::generate_graph (path, &mut graph, &crate_name, skip_length).is_err() {
                        println!("Error when generating the graph.");
                    }
                    let output = graph::format_graph (graph);
                    println!("{}", output);
                }
                None => println!("Crate name?")
            }
        }
        None => println!("Which directory? Crate name?"),
    }
}
