use std::{env, path::Path};

use cargo_graphmod::{read_files, dependencies_graph, components::CodeBase, output_for_dot};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(directory) => {
            match args.get(2) {
                Some(crate_name) => {
                    let path = Path::new(directory);
                    let skip_length = path.iter().count();
                    let mut code_to_analyze = CodeBase::new();
                    read_files::read_files(path, &mut code_to_analyze, skip_length).expect("Unable to read code base!");
                    let trie = dependencies_graph::generate_trie_from_code(&code_to_analyze, crate_name);
                    let output = output_for_dot::show(&trie);
                    println!("{}", output);
                }
                None => println!("Crate name?")
            }
        }
        None => println!("Which directory? Crate name?"),
    }
}
