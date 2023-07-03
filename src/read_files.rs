use std::{path::Path, fs::read_to_string, io::Result};

use crate::components::{ModuleComponents, CodeBase};

pub fn read_files(path: &Path, code: &mut CodeBase, skip_length: usize) -> Result<()> {
    if path.is_file() {
        if let Some(Some("rs")) = path.extension().map(|e| e.to_str()) {
            let contents = read_to_string(path)?;
            let components: ModuleComponents = path.with_extension("").iter()
                .skip(skip_length)
                .map(|s| s.to_string_lossy().into())
                .filter(|s| s != "mod")
                .collect::<Vec<String>>().into();
            code.0.insert(components, contents);
        }
    } else if path.is_dir() {
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                read_files(&entry.path(), code, skip_length)?;
            }
        }
    }
    Ok(())
}

