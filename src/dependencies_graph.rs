/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */

use crate::{trie::Trie, dependency_components::DependencyComponents};

pub type DependenciesGraph = Trie<String, Vec<DependencyComponents>>;
