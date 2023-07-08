/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */

use std::fs::read_to_string;

use cargo_graphmod::app_builder::run_app;

#[test]
fn it_generates_the_web_app_graph() {
    let output = run_app("tests/web_app/src", "web_app");
    let golden_master = read_to_string("tests/web_app/modules.dot").unwrap();
    assert_eq!(output.trim(), golden_master.trim());
}