/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
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

    pub fn components(&self) -> & Vec<String> {
        &self.components
    }

    pub fn prefix(&self) -> &Option<Vec<String>> {
        &self.prefix
    }
}
