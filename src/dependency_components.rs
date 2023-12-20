/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DependencyComponents {
    pub components: Vec<String>,
    pub file_path: Option<Vec<String>>, // for external dependencies
}

impl DependencyComponents {
    pub fn new(components: Vec<String>, file_path: Option<Vec<String>>) -> Self {
        Self { components, file_path }
    }

    /*pub fn components(&self) -> &Vec<String> {
        &self.components
    }

    pub fn prefix(&self) -> &Option<Vec<String>> {
        &self.prefix
    }*/
}
