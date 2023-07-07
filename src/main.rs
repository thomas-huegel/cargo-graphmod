use std::env;

use cargo_graphmod::app_builder::run_app;

const SRC: &str = "src";

fn main() {
    match env::var("CARGO_PKG_NAME") {
        Ok(pkg_name) => {
            let output = run_app(SRC, &pkg_name);
            println!("{}", output);
        }
        Err(_) => println!("Unable to determine package name from CARGO_PKG_NAME.")
    }
}
