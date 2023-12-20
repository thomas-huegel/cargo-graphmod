/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */
use std::{env, path::Path};

use cargo_graphmod::app_builder::run_app;

const GRAPHMOD: &str = "graphmod";
const SRC: &str = "src";

fn basename(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    let (directory, pkg_name) = match args.get(1) {
        Some(dir) if dir != GRAPHMOD => (dir.to_string() + "/" + SRC, basename(&Path::new(dir))),
        _ => (
            SRC.to_string(),
            match args.get(2) {
                Some(name) => name.to_string(),
                None => basename(&env::current_dir().unwrap()),
            },
        ),
    };
    let pkg_rust_name = pkg_name.replace('-', "_");
    let output = run_app(&directory, &pkg_rust_name);
    println!("{}", output);
}
