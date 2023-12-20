/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */
use crate::{dependency_components::DependencyComponents, trie::Trie};

pub type DependenciesGraph = Trie<String, Vec<DependencyComponents>>;
