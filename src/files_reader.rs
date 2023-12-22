/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */
use std::{collections::VecDeque, fs::read_to_string, io::Result, path::Path};

use crate::{dependencies_graph::DependenciesGraph, parser::Parser};

const EXTENSION: &str = "rs";

pub fn build_dependencies_trie<LanguageParser: Parser>(
    path: &Path,
    trie: &mut DependenciesGraph,
    skip_length: usize,
) -> Result<()> {
    if path.is_file() {
        if let Some(Some(EXTENSION)) = path.extension().map(|e| e.to_str()) {
            let contents = read_to_string(path)?;
            let components = path
                .with_extension("")
                .iter()
                .skip(skip_length)
                .map(|s| s.to_string_lossy().into())
                .collect::<VecDeque<_>>();
            let dependencies = LanguageParser::parse_dependencies(&contents);
            trie.insert(components.clone(), dependencies);
        }
    } else if path.is_dir() {
        for entry in path.read_dir().expect("read_dir call failed").flatten() {
            build_dependencies_trie::<LanguageParser>(&entry.path(), trie, skip_length)?;
        }
    } else {
        read_to_string(path)?;
    }
    Ok(())
}
