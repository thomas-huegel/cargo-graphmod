use std::env;

use cargo_graphmod::app_builder::run_app;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(directory) => {
            match args.get(2) {
                Some(crate_name) => {
                    let output = run_app(directory, crate_name);
                    println!("{}", output);
                }
                None => println!("Crate name?")
            }
        }
        None => println!("Which directory? Crate name?"),
    }
}
