/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DependencyComponents {
    components: Vec<String>,
    prefix: Option<Vec<String>>,
}

impl DependencyComponents {
    pub fn new(components: Vec<String>, prefix: Option<Vec<String>>) -> Self {
        Self {
            components,
            prefix,
        }
    }

    pub fn components<'a>(&'a self) -> &'a Vec<String> {
        &self.components
    }

    pub fn prefix<'a>(&'a self) -> &'a Option<Vec<String>> {
        &self.prefix
    }
}
