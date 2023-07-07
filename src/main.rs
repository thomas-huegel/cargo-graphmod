/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

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
