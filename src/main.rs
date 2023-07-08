/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */

use std::env;

use cargo_graphmod::app_builder::run_app;

const SRC: &str = "src";

fn main() {
    let args: Vec<String> = env::args().collect();
    let pkg_name = match args.get(1) {
        Some (name) => name.to_string(),
        None => env::current_dir().expect("Unable to read current directory.")
            .file_name().expect("Unable to read current directory name")
            .to_str().expect("Unable to convert current directory name to string")
            .to_string()
    };
    let pkg_rust_name = pkg_name.replace('-', "_");
    let output = run_app(SRC, &pkg_rust_name);
    println!("{}", output);
}