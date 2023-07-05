use std::{path::Path, fs::read_to_string, io::Result, collections::VecDeque};

use crate::{dependencies_parser, dependencies_graph::DependenciesGraph};

const EXTENSION: &str = "rs";

pub fn read_files<'a>(path: &Path, trie: &mut DependenciesGraph, skip_length: usize, crate_name: &str) -> Result<()> {
    if path.is_file() {
        if let Some(Some(EXTENSION)) = path.extension().map(|e| e.to_str()) {
            let contents = read_to_string(path)?;
            let components = path.with_extension("").iter()
                .skip(skip_length)
                .map(|s| s.to_string_lossy().into())
                .collect::<VecDeque<_>>();
            trie.insert(components.clone(), 
                dependencies_parser::parse_dependencies(&contents, crate_name, components.into()));
        }
    } else if path.is_dir() {
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                read_files(&entry.path(), trie, skip_length, crate_name)?;
            }
        }
    }
    Ok(())
}
