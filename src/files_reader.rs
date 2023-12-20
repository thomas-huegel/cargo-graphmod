/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */
use std::{collections::VecDeque, fs::read_to_string, io::Result, path::Path};

use crate::{dependencies_graph::DependenciesGraph, dependencies_parser};

const EXTENSION: &str = "rs";

pub fn read_files(
    path: &Path,
    trie: &mut DependenciesGraph,
    skip_length: usize,
    pkg_name: &str,
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
            trie.insert(
                components.clone(),
                dependencies_parser::parse_dependencies(&contents, pkg_name, components.into()),
            );
        }
    } else if path.is_dir() {
        for entry in path.read_dir().expect("read_dir call failed").flatten() {
            read_files(&entry.path(), trie, skip_length, pkg_name)?;
        }
    } else {
        read_to_string(path)?;
    }
    Ok(())
}
