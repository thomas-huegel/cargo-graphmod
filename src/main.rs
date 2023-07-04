use std::{env, path::Path};

use cargo_graphmod::{read_files, output_for_dot, components::DependenciesGraph};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(directory) => {
            match args.get(2) {
                Some(crate_name) => {
                    let path = Path::new(directory);
                    let skip_length = path.iter().count();
                    let mut trie = DependenciesGraph::new();
                    read_files::read_files(path, &mut trie, skip_length, &crate_name).expect("Unable to read code base!");
                    let output = output_for_dot::show(&trie);
                    println!("{}", output);
                }
                None => println!("Crate name?")
            }
        }
        None => println!("Which directory? Crate name?"),
    }
}
